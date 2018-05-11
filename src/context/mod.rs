use glium::Display;

#[derive(Clone)]
pub struct Context {
    pub(crate) display: Display
}


impl Context {
    pub fn new(display: Display) -> Context {
        Context {
            display
        }
    }
}
