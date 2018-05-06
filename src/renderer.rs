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

use trap::Vector2;


use std::str::from_utf8;

use Color;
use shapes::Rectangle;


pub struct Renderer<'a> {
    // The display to render to
    display: Display,

    // A shader program
    program: Program,

    // Parameters to use while drawing
    draw_parameters: DrawParameters<'a>,


    // The frame to render to
    frame: Option<Frame>,

    // The color used for filling shapes
    fill_color: Color
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

        Renderer {
            display,

            program,
            draw_parameters: DrawParameters::default(),

            frame: None,

            fill_color: Color::grey(1.0),
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
            frame.clear_color_srgb(color.r, color.g, color.b, color.a);
        } else {
            panic!("Renderer: Attempted to draw before calling 'begin'");
        }
    }



    fn draw_vertices(&mut self, vertices: &[Vertex], indices: &[u32], primitive: PrimitiveType) {
        if let Some(ref mut frame) = self.frame {
            let vertex_buffer = VertexBuffer::new(&self.display, vertices).unwrap();
            let index_buffer = IndexBuffer::new(&self.display, primitive, indices).unwrap();

            use glium;
            let uniforms = glium::uniforms::EmptyUniforms;

            frame.draw(&vertex_buffer, &index_buffer, &self.program, &uniforms, &self.draw_parameters).unwrap();
        }
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



    /// Creates a new vertex based on the current state
    fn new_vertex(&self, position: Vector2) -> Vertex {
        Vertex {
            position: position.into(),
            color: self.fill_color.into(),
        }
    }



    /// Renders a filled rectangle
    pub fn fill_rectangle(&mut self, rectangle: Rectangle) {
        let vertices = [
            self.new_vertex(Vector2::new(rectangle.left, rectangle.top)),
            self.new_vertex(Vector2::new(rectangle.right, rectangle.top)),
            self.new_vertex(Vector2::new(rectangle.right, rectangle.bottom)),
            self.new_vertex(Vector2::new(rectangle.left, rectangle.bottom)),
        ];

        let indices = [
            0, 1, 2,
            2, 3, 0
        ];

        self.draw_vertices(&vertices, &indices, PrimitiveType::TrianglesList);
    }

    /// Renders multiple points
    pub fn draw_points(&mut self, points: &[Vector2]) {
        let vertices: Vec<Vertex> = points.iter().map(|p|{
            self.new_vertex(*p)
        }).collect();

        self.draw_vertices(
            &vertices, (0..vertices.len() as u32).collect::<Vec<u32>>().as_slice(), PrimitiveType::Points
        );
    }



    /// Renders a line
    pub fn draw_line(&mut self, start: Vector2, end: Vector2) {
        let vertices = [
            self.new_vertex(start),
            self.new_vertex(end)
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
    pub color: [f32; 4]
}


implement_vertex!(Vertex, position, color);