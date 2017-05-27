extern crate sdl2;

pub trait Component {
    fn update(&mut self);
    fn draw(&self);
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
        pub fn draw(&self, renderer: &mut sdl2::render::WindowCanvas) {
            let _ = renderer.clear();

            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub fn new() -> Screen {
        Screen {
            components: Vec::new()
        }
    }
}
