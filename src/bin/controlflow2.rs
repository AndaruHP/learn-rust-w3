#![allow(unused)]

fn main() {
    // match
    let x = 14;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => {}
    }

    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        // 4 until 10
        // if not until just 4..10
        4..=10 => println!("> 3"),
        i @ 11..16 => println!("<{}>", i),
        _ => {}
    }

    let x: Option<i32> = Some(10);
    match x {
        Some(value) => println!("value = {}", value),
        None => println!("None")
    }

    let res: Result<i32, String> = Ok(10);
    match res {
        Ok(value) => println!("value = {}", value),
        Err(e) => println!("Got error: {}", e)
    }

    let x: Option<i32> = Some(10);
    let z: i32 = match x {
        Some(val) => val,
        None => 0
    };
    println!("z = {}", z);

    // another version
    if let Some(val) = x {
        println!("x = {}", val);
        // cuz if it's None, we do nothing
    }


}