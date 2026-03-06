#![allow(unused)]

#[derive(Debug)]
// struct
struct Point {
    x: i32,
    y: i32,
}

struct Point3D(i32, i32, i32);

struct Empty;

#[derive(Debug)]
struct Circle {
    radius: i32,
    center: Point
}

fn main() {
    let p = Point{x:0, y:0};
    println!("{:?}", p);
    println!("X: {}, Y: {}", p.x, p.y);

    let p = Point3D(-1, 2, 3);
    println!("point 3D: {} {} {}", p.0, p.1, p.2);

    let empty = Empty;

    let cirlce = Circle {
        radius: 1,
        center: Point{x:0, y:0}
    };

    let x: i32 = 1;
    let y: i32 = 2;
    let p = Point{x: x, y: y};
    let p = Point{x, y};

    let p0 = Point{x:1, y:2};
    let p1 = Point{x:1, y:p0.y};
    let p1 = Point{x:1, ..p0};
    println!("{:?}", p);

    let mut p = Point{x: 1, y: 2};
    p.x += 1;
    p.y += 2;
    println!("{:?}", p);
}