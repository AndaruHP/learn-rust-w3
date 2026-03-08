#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    // static method
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    // method
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p = Point { x: 1.0, y: 2.0 };
    p.move_to(2.0, 5.0);
    println!("{:#?}", p);
    
    let mut p = Point::new(1.0, 2.0);
}