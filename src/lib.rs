extern crate glium;

use std::time::Instant;

mod app;
pub use app::App;

mod renderer;
pub use renderer::Renderer;

mod color;
pub use color::Color;

pub fn run_app(mut app: Box<App>) {
    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Aurora");

    let context = glium::glutin::ContextBuilder::new()
        .with_srgb(true);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut renderer = Renderer::new(display.clone());

    app.init();


    let mut running = true;
    let mut previous_instant = Instant::now();

    while running {
        events_loop.poll_events(|e|{
            use glium::glutin::Event;
            match e {
                Event::WindowEvent { event, .. } => {
                    use glium::glutin::WindowEvent;
                    match event {
                        WindowEvent::Resized(_, _) => {},
                        WindowEvent::Moved(_, _) => {},
                        WindowEvent::Closed => {
                            running = false
                        },
                        WindowEvent::DroppedFile(_) => {},
                        WindowEvent::HoveredFile(_) => {},
                        WindowEvent::HoveredFileCancelled => {},
                        WindowEvent::ReceivedCharacter(_) => {},
                        WindowEvent::Focused(_) => {},
                        WindowEvent::KeyboardInput { .. } => {},
                        WindowEvent::CursorMoved { .. } => {},
                        WindowEvent::CursorEntered { .. } => {},
                        WindowEvent::CursorLeft { .. } => {},
                        WindowEvent::MouseWheel { .. } => {},
                        WindowEvent::MouseInput { .. } => {},
                        WindowEvent::TouchpadPressure { .. } => {},
                        WindowEvent::AxisMotion { .. } => {},
                        WindowEvent::Refresh => {},
                        WindowEvent::Touch(_) => {},
                        WindowEvent::HiDPIFactorChanged(_) => {},
                    }
                },

                _ => ()
            }
        });

        {
            let current_instant = Instant::now();
            let duration = current_instant - previous_instant;
            let delta_time = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
            app.update(delta_time);

            previous_instant = current_instant;
        }

        renderer.begin();
        app.render(&mut renderer);
        renderer.end();

        display.swap_buffers().unwrap();
    }
}
