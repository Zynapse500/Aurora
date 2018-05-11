use std::rc::Rc;

use std::path::Path;

use Color;
use color::ColorImageData;
use context::Context;

use glium::texture::texture2d::Texture2d;
use glium::texture::RawImage2d;
use glium::uniforms::Sampler;


use image::open;



#[derive(Clone)]
pub struct Texture {
    texture: Rc<Texture2d>
}


impl Texture {
    pub fn from_colors(context: &Context, colors: &[Color], width: u32, height: u32) -> Texture {
        Texture {
            texture: Rc::new(
                Texture2d::new(
                    &context.display,
                    ColorImageData(colors, width, height)
                ).unwrap()
            )
        }
    }

    pub fn from_file<P: AsRef<Path>>(context: &Context, path: P) -> Result<Texture, ()> {
        let image= match open(path) {
            Ok(image) => image,
            Err(_) => return Err(())
        };
        let rgba = image.to_rgba();

        Ok(Texture {
            texture: Rc::new(
                Texture2d::new(
                    &context.display,
                    RawImage2d::from_raw_rgba(rgba.to_vec(), (rgba.width(), rgba.height()))
                ).unwrap()
            )
        })
    }

    pub(crate) fn sampled(&self) -> Sampler<Texture2d> {
        self.texture.sampled()
    }


    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }
}
