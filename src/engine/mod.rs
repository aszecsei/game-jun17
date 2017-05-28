extern crate sdl2;
extern crate time;
use std::time::Instant;
use std::cmp::max;

use sdl2::event::{Event,WindowEvent};
use sdl2::keyboard::Keycode;

pub mod input;
pub mod graphics;
pub mod math;

pub struct Game<'t> {
    pub events: sdl2::EventPump,
    pub screen: graphics::Screen,
    pub renderer: graphics::renderer::Renderer<'t>,
    pub delta_time: f64,
    timer_subsystem: sdl2::TimerSubsystem,
    pub screen_width: u32,
    pub screen_height: u32
}

impl<'t> Game<'t> {
    pub fn start(&mut self) {
        // loop until we receive a QuitEvent
        // loop until we receive a QuitEvent
        let mut last_time: u32;
        let mut current_time = self.timer_subsystem.ticks();
        let target_frame_time: u32 = 1000/100; // 100fps = 10ms
        'event : loop {
            // poll_event returns the most recent event or NoEvent if nothing has happened
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit{..} => break 'event,
                    Event::Window {win_event, ..} => {
                        match win_event {
                            // refresh our window, for example if it is no longer
                            // covered by other windows
                            WindowEvent::Exposed => self.renderer.window.present(),
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
            last_time = current_time;
            current_time = self.timer_subsystem.ticks();
            self.delta_time = current_time as f64 - last_time as f64;
            self.screen.update(self.delta_time);
            self.screen.draw(&mut self.renderer);
             //not sleeping for the whole time gives a smoother result
            let sleep_time = max(0, target_frame_time as i32 - (current_time as i32 - last_time as i32));
            self.timer_subsystem.delay(sleep_time as u32 / 2);
        }
    }

    pub fn set_screen(&mut self, new_screen: graphics::Screen) {
        self.screen = new_screen;
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
    let timer = ctx.timer().unwrap();

    return Game {
        events: events,
        screen: graphics::Screen { objects: Vec::new() },
        renderer: graphics::renderer::Renderer::new(renderer),
        delta_time: 0.0,
        timer_subsystem: timer,
        screen_width: disp.w as u32,
        screen_height: disp.h as u32
    };
}

pub trait GameObject {
    fn update(&mut self, delta_time: f64);
    fn draw(&self, &mut graphics::renderer::Renderer);
}
