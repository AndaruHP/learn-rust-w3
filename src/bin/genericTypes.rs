#![allow(unused)]

// enum Option<T> {
//     Some(T),
//     None
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

struct Point<T> {
    x: T,
    y: T
}

fn swap<A, B>(a: A, b: B) -> (B, A) {
// fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    // T = u32
    let v: Vec<u32> = vec![1u32, 2, 3, 4];

    let p: Point<u32> = Point { x: 1, y: 2 };

    // let mut a = 1;
    // let mut b = 2;
    // (a, b) = swap(a, b);
    
    let a: i32 = 1;
    let b: u32 = 2;
    let (a, b) = swap(a, b);
}