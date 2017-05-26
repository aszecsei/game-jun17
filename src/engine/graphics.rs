extern crate sdl2;

pub trait Component {
    fn update(&mut self);
    fn draw(&self, &sdl2::render::WindowCanvas);
}

pub mod screen {
    extern crate sdl2;
    use engine::graphics::Component;

    pub struct Screen {
        pub components: Vec<Box<Component>>,
    }

    impl Screen {
        pub fn update(&mut self) {
            for component in self.components.iter_mut() {
                component.update();
            }
        }
        pub fn draw(&self, renderer: &sdl2::render::WindowCanvas) {
            for component in self.components.iter() {
                component.draw(renderer);
            }
        }
    }

    pub fn new() -> Screen {
        Screen {
            components: Vec::new()
        }
    }
}

pub mod sprite {
    extern crate sdl2;
    use engine::graphics::Component;

    pub struct Sprite {
        // TODO: Add behaviors here
    }

    impl Component for Sprite {
        fn update(&mut self) {

        }

        fn draw(&self, renderer: &sdl2::render::WindowCanvas) {
            // TODO: Draw the sprite
        }
    }
}
