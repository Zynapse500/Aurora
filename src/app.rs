use Renderer;

pub trait App {
    fn init(&mut self) {}
    fn update(&mut self, delta_time: f64);

    fn render(&mut self, renderer: &mut Renderer);
}


