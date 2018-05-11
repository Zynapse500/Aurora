use glium::texture::Texture2dDataSource;
use glium::texture::RawImage2d;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn grey(grey: f32) -> Color {
        Color {
            r: grey,
            g: grey,
            b: grey,
            a: 1.0,
        }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            r,
            g,
            b,
            a: 1.0,
        }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }
}



impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.r as f32,
            self.g as f32,
            self.b as f32,
            self.a as f32
        ]
    }
}



pub(crate) struct ColorImageData<'a> (pub &'a [Color], pub u32, pub u32);

impl<'a> Texture2dDataSource<'a> for ColorImageData<'a> {
    type Data = f32;

    fn into_raw(self) -> RawImage2d<'a, f32> {
        let mut colors = Vec::new();
        colors.reserve(self.0.len() * 4);

        for color in self.0 {
            colors.push(color.r);
            colors.push(color.g);
            colors.push(color.b);
            colors.push(color.a);
        }

        RawImage2d::from_raw_rgba(colors, (self.1, self.2))
    }
}
