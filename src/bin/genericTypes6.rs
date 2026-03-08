#![allow(unused)]

// Lifetime
// every reference has a lifetime

// fn longest_str(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("{} {}", x, y);
}

#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
}

impl<'a> Book<'a> {
    fn edit(&mut self, new_title: &'a str) {
        self.title = new_title;
    }
}

fn main() {
    let x = "Hello";
    let y = "World";
    let z = longest_str(&x, &y);
    println!("longest {:#?}", z);

    // static lifetime
    let s: &'static str = "Hello";

    // placeholder lifetime - let rust infer the lifetime
    let s: &'_ str = "Rust";
}