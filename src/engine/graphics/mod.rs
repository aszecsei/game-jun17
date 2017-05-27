extern crate sdl2;

pub mod renderer;

pub trait Drawable {
    fn draw(&self, &mut renderer::Renderer);
}

pub trait Updatable {
    fn update(&mut self);
}

pub trait Component: Drawable + Updatable {
}

pub struct Screen {
    pub components: Vec<Box<Component>>,
}

impl Screen {
    pub fn update(&mut self) {
        for component in self.components.iter_mut() {
            component.update();
        }
    }
    pub fn draw(&self, renderer: &mut renderer::Renderer) {
        let _ = renderer.window.clear();

        for component in self.components.iter() {
            component.draw(renderer);
        }
    }

    pub fn new() -> Screen {
        Screen {
            components: Vec::new()
        }
    }
}
