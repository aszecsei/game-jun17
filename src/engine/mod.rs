extern crate sdl2;

use sdl2::event::{Event,WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator, Texture};
use sdl2::surface::{Surface};

use std::collections::HashMap;
use std::path::Path;

pub mod input;
pub mod graphics;
pub mod math;

pub struct Game<'t> {
    pub renderer: sdl2::render::WindowCanvas,
    pub events: sdl2::EventPump,
    pub screen: graphics::screen::Screen,
    textures: HashMap<&'t str, Texture<'t>>
}

impl<'t> Game<'t> {
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
            self.screen.draw(&mut self.renderer);
        }
    }

    pub fn set_screen(&mut self, new_screen: graphics::screen::Screen) {
        self.screen = new_screen;
    }

    pub fn load_texture(&mut self, texture_creator: &'t mut TextureCreator<sdl2::video::WindowContext>, name: &'t str, path: &'t str) {
        // Load a surface.
        // Surfaces live in system RAM, so they aren't ideal for performance.
        let surface = match Surface::load_bmp(&Path::new(path)) {
            Ok(surface) => surface,
            Err(err)    => panic!("failed to load surface: {}", err)
        };

        // Convert a surface to a texture.
        // Textures can be used more efficiently by the GPU. (If one is available.)
        let texture = match texture_creator.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(err)    => panic!("failed to convert surface: {:?}", err)
        };

        self.textures.insert(name, texture);
    }

    pub fn draw_texture(&mut self, name: &'t str, src: Option<math::Rect>, dst: Option<math::Rect>) {
        // Display the texture.
        // Omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer.
        // Try passing Some(surface.rect()) for src & dst instead of None & see how things change.
        let texture = &self.textures[name];
        let srcSDL = src.map(|s| sdl2::rect::Rect::new(s.x(), s.y(), s.width(), s.height()));
        let dstSDL = dst.map(|s| sdl2::rect::Rect::new(s.x(), s.y(), s.width(), s.height()));
        let _ = self.renderer.copy(&texture, srcSDL, dstSDL);
        let _ = self.renderer.present();
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
        screen: graphics::screen::Screen { components: Vec::new() },
        textures: HashMap::new()
    };
}
