use glium::Frame;
use glium::Display;
use glium::Surface;

use Color;


pub struct Renderer {
    display: Display,

    frame: Option<Frame>
}

impl Renderer {
    pub fn new(display: Display) -> Renderer {
        Renderer {
            display,

            frame: None,
        }
    }


    pub(crate) fn begin(&mut self) {
        if self.frame.is_none() {
            self.frame = Some(self.display.draw());
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


    pub fn clear(&mut self, color: Color) {
        if let Some(ref mut frame) = self.frame {
            frame.clear_color_srgb(color.r, color.g, color.b, color.a);
        } else {
            panic!("Renderer: Attempted to draw before calling 'begin'");
        }
    }
}
