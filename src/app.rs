use Renderer;
use context::Context;

pub use glium::glutin::VirtualKeyCode as KeyCode;
pub use glium::glutin::MouseButton;

pub trait App {

    /*
     * Initialization
     */

    // Called before the app starts running
    #[allow(unused_variables)]
    fn init(&mut self, context: Context);


    /*
     * Main loop
     */

    // Updates the app
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64);


    // Renders the app
    #[allow(unused_variables)]
    fn render(&mut self, renderer: &mut Renderer);


    /*
     * Event Handling
     */

    // Called when the size of the window changes
    #[allow(unused_variables)]
    fn size_changed(&mut self, width: u32, height: u32) {}


    // Called when a key is pressed
    #[allow(unused_variables)]
    fn key_pressed(&mut self, key: KeyCode) {}

    // Called when a key is released
    #[allow(unused_variables)]
    fn key_released(&mut self, key: KeyCode) {}



    // Called when the cursor moves
    #[allow(unused_variables)]
    fn cursor_moved(&mut self, x: f64, y: f64) {}


    // Called when a mouse button is pressed
    #[allow(unused_variables)]
    fn mouse_pressed(&mut self, button: MouseButton) {}

    // Called when a mouse button is released
    #[allow(unused_variables)]
    fn mouse_released(&mut self, button: MouseButton) {}
}


