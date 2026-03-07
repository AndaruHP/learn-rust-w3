#![allow(unused)]

fn main() {
    let x: Option<i32> = Some(3);
    let v: i32 = match x {
        Some(value) => value,
        None => panic!("no value")
    };

    // unwrap the inner value. Panic if None
    let i = x.unwrap();
    println!("{}", i);

    let x: Result<i32, String> = Err("something went wrong".to_string());
    let v: i32 = match x {
        Ok(value) => value,
        Err(e) => panic!("{}", e)
    };

    let x: Result<i32, String> = Err("something went wrong".to_string());
    x.expect("something went wrong");
}