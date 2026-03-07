#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other
}

fn main() {
    // panic!("Something went wrong");

    let v = vec![1,2,3];
    let x: Option<&i32> = v.get(0);
    match x {
        Some(value) => println!("Got value: {}", value),
        None => println!("None")
    }

    let x = 1;
    let y = 0;
    // panic. Division by 0
    // let q = x / y;


    let q: Result<i32, MathError> = if y != 0 {
        Ok(x / y)
    } else {
        Err(MathError::DivByZero)
    };

    match q {
        Ok(value) => println!("value: {}", value),
        Err(e) => println!("Error: {:?}", e)
    }
}