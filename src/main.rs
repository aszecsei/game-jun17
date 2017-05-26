extern crate sdl2;

use sdl2::event::{Event,WindowEvent};
use sdl2::keyboard::Keycode;

mod engine;

fn main() {
    let mut game = engine::init();

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        for event in game.events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::Window {win_event, ..} => {
                    match win_event {
                        // refresh our window, for example if it is no longer
                        // covered by other windows
                        WindowEvent::Exposed => game.renderer.present(),
                        _                      => (),
                    }
                }
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => break 'event,
                        _               => (),
                    }
                }
                _               => continue
            }
        }
    }
}
