#![allow(unused)]

// constant
// cuz the value cannot change
// we can add it out of main fn
const NUM: u32 = 1;

fn main() {
    println!("Hello, world!");

    // variable in rust is immutable
    // that's why we need to add mut
    let mut x = 1;
    x += 1;
    // also we don't need to declare
    // the type of the variable
    let y: i32 = -1;
    let z = -1;
    // shadowing
    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;
    // let rust decide the type
    let x: _ = 123;

    // print
    let x = 1;
    println!("x is {}", x);
    println!("x is {x}");
    // print but like an index
    println!("{0} + {0} = {1}", x, x + x);
    // debug
    // for one straight line
    // User {name: "Santos"}
    print!("x {:?}", x);
    // if like a struct, so it will
    // print in multiple lines
    // User {
    //   name: "Santos"
    // }
    print!("x {:#?}", x);

    let x = 1;
    let y = 2;
    let z = add(x, y);
    let z1 = add_with_return(x, y);

    print!("z = {z}");
    print!("z1 = {z1}");
    print("🎌".to_string());
}

// two approaches for a function
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn add_with_return(x: i32, y: i32) -> i32 {
    return x + y;
}

fn print(s: String) {
    println!("{s}{s}{s}{s}{s}");
}
