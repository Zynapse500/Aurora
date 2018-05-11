#[macro_use]
extern crate glium;

extern crate trap;


extern crate image;



pub use trap::*;

use std::time::Instant;


mod context;
pub use context::Context;

mod app;
pub use app::App;
pub use app::KeyCode;

mod renderer;
pub use renderer::Renderer;
pub use renderer::Render;
pub use renderer::PolygonMode;

mod color;
pub use color::Color;

mod shapes;
pub use shapes::Rectangle;
pub use shapes::Circle;
pub use shapes::ConvexHull;


mod texture;
pub use texture::Texture;


mod frame_counter;
pub use frame_counter::FrameCounter;



use glium::glutin::KeyboardInput;
use glium::glutin::ElementState;
use std::collections::HashSet;


pub fn run_app(mut app: Box<App>, width: u32, height: u32, title: &str) {
    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        //.with_fullscreen(Some(events_loop.get_primary_monitor()))
        .with_title(title);

    let context = glium::glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(false);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut renderer = Renderer::new(display.clone());


    app.init(Context::new(display.clone()));


    let mut running = true;
    let mut previous_instant = Instant::now();
    let mut pressed_keys = HashSet::new();

    loop {
        events_loop.poll_events(|e|{
            use glium::glutin::Event;
            match e {
                Event::WindowEvent { event, .. } => {
                    use glium::glutin::WindowEvent;
                    match event {
                        WindowEvent::Closed => {
                            running = false
                        },

                        WindowEvent::Resized(w, h) => {
                            app.size_changed(w, h);
                        },
                        WindowEvent::Moved(_, _) => {},

                        WindowEvent::DroppedFile(_) => {},
                        WindowEvent::HoveredFile(_) => {},
                        WindowEvent::HoveredFileCancelled => {},
                        WindowEvent::ReceivedCharacter(_) => {},
                        WindowEvent::Focused(_) => {},
                        WindowEvent::KeyboardInput { input: KeyboardInput {
                            state, virtual_keycode, ..
                        }, .. } => {
                            if let Some(key_code) = virtual_keycode {
                                match state {
                                    ElementState::Pressed => {
                                        if pressed_keys.insert(key_code) {
                                            app.key_pressed(key_code);
                                        }
                                    },
                                    ElementState::Released => {
                                        pressed_keys.remove(&key_code);
                                        app.key_released(key_code);
                                    },
                                }
                            }
                        },
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

        if running {
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
        } else {
            break;
        }
    }
}
