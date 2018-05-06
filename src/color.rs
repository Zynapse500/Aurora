
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