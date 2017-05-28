extern crate sdl2;

use sdl2::render::{TextureCreator, Texture};
use sdl2::surface::{Surface};

use std::collections::HashMap;
use std::path::Path;

use engine::math;

pub struct Renderer<'t> {
    pub window: sdl2::render::WindowCanvas,
    textures: HashMap<&'t str, Texture<'t>>
}

impl<'t> Renderer<'t> {
    pub fn new(win: sdl2::render::WindowCanvas) -> Renderer<'t> {
        Renderer {
            window: win,
            textures: HashMap::new()
        }
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
        let src_sdl = src.map(|s| sdl2::rect::Rect::new(s.x(), s.y(), s.width(), s.height()));
        let dst_sdl = dst.map(|s| sdl2::rect::Rect::new(s.x(), s.y(), s.width(), s.height()));
        let _ = self.window.copy(&texture, src_sdl, dst_sdl);
        let _ = self.window.present();
    }
}
