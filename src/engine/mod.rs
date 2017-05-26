extern crate sdl2;

use sdl2::event::{Event,WindowEvent};
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};
use sdl2::keyboard::Keycode;

mod input;

pub struct Game {
    pub renderer: sdl2::render::WindowCanvas,
    pub events: sdl2::EventPump
}

pub fn init() -> Game {
    // start sdl2
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // Get the display mode
    let disp = match video_ctx.current_display_mode(0) {
        Ok(disp) => disp,
        Err(err) => panic!("failed to get display mode: {}", err)
    };

    // Create a window
    let mut window = match video_ctx.window("eg01", disp.w as u32, disp.h as u32).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let _ = window.set_bordered(false);

    // Create a rendering context
    let mut renderer = match window.into_canvas().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };

    let mut events = ctx.event_pump().unwrap();

    return Game {
        renderer: renderer,
        events: events
    };
}
