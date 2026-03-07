#![allow(unused)]

fn main() {
    // vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    let v = vec![1, 2, 3, 4];
    let v = vec![0i32; 10];
    println!("v: {:?}", v);
    println!("v: {}", v[0] + 1);

    // Option
    // index valid -> Some(&val)
    // invalid -> None
    println!("v[1]: {:?}", v.get(1));
    println!("v[1]: {:?}", v.get(20));

    let mut v: Vec<i8> = vec![1, 2, 3,];;
    let x: Option<i8> = v.pop();
    println!("x: {:?}", x);
    let x: Option<i8> = v.pop();
    println!("x: {:?}", x);
    let x: Option<i8> = v.pop();
    println!("x: {:?}", x);
    let x: Option<i8> = v.pop();
    println!("x: {:?}", x);

    // slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..4];
    println!("s: {:?}", s);
}
