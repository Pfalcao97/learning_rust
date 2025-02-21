pub trait Area {
    fn area(&self) -> f32;
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Square {
    pub side: f32,
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

pub struct Triangle {
    pub width: f32,
    pub height: f32,
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        (self.width * self.height) / 2.0
    }
}

pub struct Circle {
    pub radius: f32,
}

const PI: f32 = 3.1415;

impl Area for Circle {
    fn area(&self) -> f32 {
        PI * self.radius.powf(2.0)
    }
}
