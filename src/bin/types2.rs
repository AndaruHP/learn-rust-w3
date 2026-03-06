#![allow(unused)]

fn main() {
    // String -> when need ownership or mutability
    // &str -? read only string
    // let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let length: usize = msg.len();
    let s: &str = &msg[0..4];
    println!("{}",s);

    let s = "Hello World";
    let x: String = s.to_string();

    // Rust convert &String into &str
    let msg: String = "Hello Rust".to_string();
    // in IDE it's a error, but when run it, it works
    let msg: String = String::from("Hello Rust");
    // this one also
    print(&msg);

    let s: &str = "Hello Rust";
    print(s);

    // append String with &str
    let mut s: String = "Hello Rust".to_string();
    s += " World";
    println!("{s}");

    // string interpolation
    let lang = "Rust";
    let emoji = "🦀";
    let mut s = "Hello".to_string();
    s += " ";;
    s += lang;
    s += " ";;
    s += emoji;
    // or
    let s = format!("Hello {} {}", lang, emoji);
    println!("{s}");

}

fn print(s: &str) {
    println!("{s}");
}