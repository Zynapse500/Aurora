use Renderer;
use context::Context;

pub use glium::glutin::VirtualKeyCode as KeyCode;

pub trait App {
    #[allow(unused_variables)]
    fn init(&mut self, context: Context);

    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64);

    #[allow(unused_variables)]
    fn render(&mut self, renderer: &mut Renderer);


    #[allow(unused_variables)]
    fn size_changed(&mut self, width: u32, height: u32) {}


    #[allow(unused_variables)]
    fn key_pressed(&mut self, key: KeyCode) {}

    #[allow(unused_variables)]
    fn key_released(&mut self, key: KeyCode) {}
}


