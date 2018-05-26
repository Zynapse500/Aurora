use glium::Display;
use glium::Program;

use glium::Frame;
use glium::Surface;

use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::index::PrimitiveType;

use glium::DrawParameters;
use glium::Blend;
use glium::BlendingFunction;
use glium::LinearBlendingFactor;

use glium::uniforms::MagnifySamplerFilter;
use glium::uniforms::MinifySamplerFilter;

pub use glium::PolygonMode;


use trap::Vector2;


use std::str::from_utf8;

use Color;
use Texture;
use Context;
use shapes::Rectangle;


/// Render an object
pub trait Render<S> {
    fn fill(&mut self, object: S);
    fn draw(&mut self, object: S);
}



/// Anything that can be turned into triangles
pub trait Triangulate {
    fn get_triangles(&self) -> Triangles;
}


pub enum Triangles {
    // An array of interleaved triangles
    TriangleList(Vec<(Vector2, Vector2, Vector2)>),

    // An array of points with indices of interleaved triangles
    IndexedTriangles(Vec<Vector2>, Vec<u32>)
}



pub struct Renderer<'a> {
    // The display to render to
    display: Display,

    // A shader program
    program: Program,

    // Parameters to use while drawing
    draw_parameters: DrawParameters<'a>,


    // The current view of the rectangle
    view: Rectangle,


    // The frame to render to
    frame: Option<Frame>,

    // The color used for filling shapes
    fill_color: Color,

    // The texture used on filled shapes
    texture: Option<Texture>,
    default_texture: Texture,

    // If textures should be flipped vertically
    flip_textures: bool
}

impl<'a> Renderer<'a> {
    pub fn new(display: Display) -> Renderer<'a> {
        let vertex_source = from_utf8(include_bytes!("shaders/shader.vert")).unwrap();
        let fragment_source = from_utf8(include_bytes!("shaders/shader.frag")).unwrap();

        let program = Program::from_source(
            &display,
            vertex_source,
            fragment_source,
            None
        ).unwrap();

        let default_texture = Texture::from_colors(
            &Context::new(display.clone()),
            &[Color::grey(1.0)], 1, 1
        );

        Renderer {
            display,

            program,
            draw_parameters: DrawParameters::default(),

            view: Rectangle::new(-1.0, 1.0, 1.0, -1.0),

            frame: None,

            fill_color: Color::grey(1.0),
            texture: None,
            default_texture,
            flip_textures: false
        }
    }

    pub(crate) fn begin(&mut self) {
        if self.frame.is_none() {
            self.frame = Some(self.display.draw());

            self.draw_parameters = DrawParameters::default();
            self.draw_parameters.blend = Blend {
                color: BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: BlendingFunction::AlwaysReplace,
                constant_value: (0.0, 0.0, 0.0, 0.0),
            };
        } else {
            panic!("Renderer: 'begin' called before 'end'!");
        }
    }


    pub(crate) fn end(&mut self) {
        if let Some(frame) = self.frame.take() {
            frame.finish().unwrap();
        } else {
            panic!("Renderer: 'end' called before 'begin'")
        }
    }



    /// Clears the screen with a solid color
    pub fn clear(&mut self, color: Color) {
        if let Some(ref mut frame) = self.frame {
            frame.clear_color(color.r, color.g, color.b, color.a);
        } else {
            panic!("Renderer: Attempted to draw before calling 'begin'");
        }
    }



    fn draw_vertices(&mut self, vertices: &[Vertex], indices: &[u32], primitive: PrimitiveType) {
        if let Some(ref mut frame) = self.frame {
            let vertex_buffer = VertexBuffer::new(&self.display, vertices).unwrap();
            let index_buffer = IndexBuffer::new(&self.display, primitive, indices).unwrap();

            let texture = if let Some(ref texture) = self.texture {
                texture
            } else {
                &self.default_texture
            }.sampled()
                .magnify_filter(MagnifySamplerFilter::Linear)
                .minify_filter(MinifySamplerFilter::Linear);

            let uniforms = uniform!(
                tex0: texture,
                left:   self.view.left      as f32,
                right:  self.view.right     as f32,
                top:    self.view.top       as f32,
                bottom: self.view.bottom    as f32,
            );

            frame.draw(&vertex_buffer, &index_buffer, &self.program, &uniforms, &self.draw_parameters).unwrap();
        }
    }


    /// Sets the current view
    pub fn set_view(&mut self, view: Rectangle) {
        self.view = view;
    }


    /// Sets the current fill color
    pub fn set_color(&mut self, color: Color) {
        self.fill_color = color;
    }


    /// Sets the current size of points
    pub fn set_point_size(&mut self, size: f64) {
        self.draw_parameters.point_size = Some(size as f32);
    }


    /// Sets the current width of lines
    pub fn set_line_width(&mut self, width: f64) {
        self.draw_parameters.line_width = Some(width as f32);
    }


    /// Sets the current mode to draw polygons in
    pub fn set_polygon_mode(&mut self, mode: PolygonMode) {
        self.draw_parameters.polygon_mode = mode;
    }


    /// Sets the current texture to use when drawing shapes
    pub fn set_texture(&mut self, texture: Option<Texture>) {
        self.texture = texture;
    }


    /// Determines wether or not to flip textures vertically
    pub fn flip_textures(&mut self, flip: bool) {
        self.flip_textures = flip;
    }



    /// Creates a new vertex based on the current state
    fn new_vertex(&self, position: Vector2, tex_coord: Option<Vector2>) -> Vertex {
        Vertex {
            position: position.into(),
            color: self.fill_color.into(),
            tex_coord: if let Some(tex) = tex_coord {
                if self.flip_textures {
                    [tex.x as f32, 1.0 - tex.y as f32]
                } else {
                    [tex.x as f32, tex.y as f32]
                }
            } else {
                [0.0; 2]
            }
        }
    }


    /// Renders multiple points
    pub fn draw_points(&mut self, points: &[Vector2]) {
        let vertices: Vec<Vertex> = points.iter().map(|p|{
            self.new_vertex(*p, None)
        }).collect();

        self.draw_vertices(
            &vertices, (0..vertices.len() as u32).collect::<Vec<u32>>().as_slice(), PrimitiveType::Points
        );
    }



    /// Renders a line
    pub fn draw_line(&mut self, start: Vector2, end: Vector2) {
        let vertices = [
            self.new_vertex(start, None),
            self.new_vertex(end, None)
        ];

        self.draw_vertices(
            &vertices, &[0, 1], PrimitiveType::LinesList
        );
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coord: [f32; 2]
}


implement_vertex!(Vertex, position, color, tex_coord);



impl<'a> Render<Triangles> for Renderer<'a> {
    fn fill(&mut self, object: Triangles) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        match object {
            Triangles::TriangleList(triangles) => {
                for (a, b, c) in triangles.into_iter() {
                    let len = vertices.len() as u32;
                    vertices.push(self.new_vertex(a, None));
                    vertices.push(self.new_vertex(b, None));
                    vertices.push(self.new_vertex(c, None));

                    indices.push(len);
                    indices.push(len + 1);
                    indices.push(len + 2);
                }
            },

            Triangles::IndexedTriangles(points, triangle_indices) => {
                indices = triangle_indices;
                vertices = points.into_iter().map(|p|self.new_vertex(p, None)).collect();
            }
        }

        self.draw_vertices(
            &vertices, &indices,
            PrimitiveType::TrianglesList
        )
    }

    fn draw(&mut self, object: Triangles) {
        unimplemented!()
    }
}

