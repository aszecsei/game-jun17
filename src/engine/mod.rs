extern crate sdl2;

use sdl2::event::{Event,WindowEvent};
use sdl2::keyboard::Keycode;

pub mod input;
pub mod graphics;

pub struct Game {
    pub renderer: sdl2::render::WindowCanvas,
    pub events: sdl2::EventPump,
    pub screen: graphics::screen::Screen
}

impl Game {
    pub fn start(&mut self) {
        // loop until we receive a QuitEvent
        'event : loop {
            // poll_event returns the most recent event or NoEvent if nothing has happened
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit{..} => break 'event,
                    Event::Window {win_event, ..} => {
                        match win_event {
                            // refresh our window, for example if it is no longer
                            // covered by other windows
                            WindowEvent::Exposed => self.renderer.present(),
                            _                      => (),
                        }
                    }
                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        match keycode {
                            Keycode::Escape => break 'event,
                            _               => (), // TODO: Update current keyboard state
                        }
                    }
                    _               => continue
                }
            }
            self.screen.update();
            self.screen.draw();
        }
    }
}

pub fn new(title:&str) -> Game {
    // start sdl2
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // Get the display mode
    let disp = match video_ctx.current_display_mode(0) {
        Ok(disp) => disp,
        Err(err) => panic!("failed to get display mode: {}", err)
    };

    // Create a window
    let mut window = match video_ctx.window(title, disp.w as u32, disp.h as u32).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let _ = window.set_bordered(false);

    // Create a rendering context
    let renderer = match window.into_canvas().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };

    let events = ctx.event_pump().unwrap();

    return Game {
        renderer: renderer,
        events: events,
        screen: graphics::screen::Screen { components: Vec::new() }
    };
}
