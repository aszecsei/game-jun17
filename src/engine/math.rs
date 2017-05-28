extern crate sdl2;

pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    // TODO: Operators

    pub fn x(&self) -> f32 { self.x }
    pub fn set_x(&mut self, x: f32) { self.x = x; }
    pub fn y(&self) -> f32 { self.y }
    pub fn set_y(&mut self, y: f32) { self.y = y; }

    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x,
            y: y
        }
    }
}

pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    // TODO: Operators

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }
}

pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    // TODO: Operators

    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }

    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32
}

impl Rect {
    // TODO: Operators

    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }
}
