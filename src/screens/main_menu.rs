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
    size: math::Vector2,
    vel: math::Vector2,
}

impl GameObject for Image {
    fn update(&mut self, delta_time: f64) {
        self.pos = math::Vector2::new(self.pos.x() + self.vel.x() * delta_time as f32, self.pos.y() + self.vel.y() * delta_time as f32);
        if self.pos.x() < 0.0 || self.pos.x() + self.size.x() > 1920.0 {
            let old_x = self.vel.x();
            self.vel.set_x(-1.0 * old_x);
        }
        if self.pos.y() < 0.0 || self.pos.y() + self.size.y() > 1200.0 {
            let old_y = self.vel.y();
            self.vel.set_y(-1.0 * old_y);
        }
    }

    fn draw(&self, renderer: &mut graphics::renderer::Renderer) {
        renderer.draw_texture("owo", None, Some(math::Rect::new(self.pos.x() as i32, self.pos.y() as i32, self.size.x() as u32, self.size.y() as u32)));
    }
}

impl Image {
    pub fn new() -> Image {
        Image {
            pos: math::Vector2::new(0.0, 0.0),
            size: math::Vector2::new(300.0, 200.0),
            vel: math::Vector2::new(0.1, 0.1),
        }
    }
}
