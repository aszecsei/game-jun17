extern crate sdl2;

pub mod renderer;

use engine::GameObject;

pub struct Screen {
    pub objects: Vec<Box<GameObject>>,
}

impl Screen {
    pub fn update(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.update();
        }
    }
    pub fn draw(&self, renderer: &mut renderer::Renderer) {
        let _ = renderer.window.clear();

        for obj in self.objects.iter() {
            obj.draw(renderer);
        }
    }

    pub fn new() -> Screen {
        Screen {
            objects: Vec::new()
        }
    }
}
