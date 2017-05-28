use engine::graphics;

use engine::math;
use engine::GameObject;

pub fn new() -> graphics::Screen {
    let mut result = graphics::Screen::new();

    // TODO: Add content
    result.objects.push(Box::new(Image::new()));

    return result;
}

pub struct Image {
    pos: math::Vector2,
    size: math::Vector2
}

impl GameObject for Image {
    fn update(&mut self, delta_time: f64) {
        self.pos = math::Vector2::new(self.pos.x() + 0.01 * delta_time as f32, self.pos.y() + 0.01 * delta_time as f32);
    }

    fn draw(&self, renderer: &mut graphics::renderer::Renderer) {
        renderer.draw_texture("owo", None, Some(math::Rect::new(self.pos.x() as i32, self.pos.y() as i32, self.size.x() as u32, self.size.y() as u32)));
    }
}

impl Image {
    pub fn new() -> Image {
        Image {
            pos: math::Vector2::new(0.0, 0.0),
            size: math::Vector2::new(300.0, 200.0)
        }
    }
}
