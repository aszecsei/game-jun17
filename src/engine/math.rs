extern crate sdl2;

pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    // TODO: Operators
}

pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    // TODO: Operators
}

pub struct Point {
    x: i32,
    y: i32
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
}