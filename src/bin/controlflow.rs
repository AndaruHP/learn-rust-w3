#![allow(unused)]
#![allow(unused_comparisons)]

fn main() {
    // if else
    let x: u32 = 10;
    let z: i32 = if x > 0 {
        println!("x > 0");
        // return without "return"
        1
    } else if x < 0 {
        println!("x < 0");
        -1
    } else {
        println!("x = 0");
        0
    };

    println!("z: {}", z);

    // loop
    let mut i = 0;
    loop {
        println!("loop {}", i);
        i += 1;
        if i > 5 {
            break;
        }
    }

    let mut i = 0;
    while i < 5 {
        println!("while loop {}", i);
        i += 1;
    }

    for i in 0..6 {
        println!("for loop {}", i);
    }

    let arr = [1, 2, 3, 4, 5];
    let n: usize = arr.len();
    for i in 0..n {
        println!("arr {i}: {}", arr[i]);
    }

    for n in arr {
        println!("{}", n);
    }

    let v = vec![1, 2, 3, 4, 5];
    // for n in v {
    //     println!("vec {}", n);
    // }
    // after using v in loop, it has problem
    // with ownership, so we have to add .iter
    // i mean after v used, it can be used again

    for n in v.iter() {
        println!("vec {}", n);
    }

    for n in v.iter() {
        println!("vec {}", n);
    }

    let mut i = 0;
    let z: &str = loop {
        println!("loop {}", i);
        i += 1;
        if i > 5 {
            break "loop ends here";
        }
    };
    println!("z: {}", z);
}