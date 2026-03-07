#![allow(unused)]

// Stack
// stores data of fixed size
// known at compile time, fast, LIFO

// Heap
// Sores data of unknown size at compile
// time, slower than Stack, Memory safety
// is enforced through Rust's ownership
// and borrowing rules
fn main() {
    // Stack
    let x: i32 = 1;
    let arr: [i32; 10] = [1; 10];

    // Heap
    let mut s: String = "Hello ".to_string();
    s += "World!";
    let mut v = vec![];
    v.push(0);
    v.push(0);
    v.push(0);

    // store Stack on the Heap
    let boxed = Box::new(1i32);

    // Ownership rules
    // 1. each value has an owner, 2. there
    // can only be one owner at a time, 3.
    // when the owner goes out of scope,
    // the value will be dropped

    // 1
    // owner of s is s
    let s = "Hello ".to_string();
    // owner of i is i
    let i = 1;

    // 2
    let s = "dog".to_string();
    // owner of s is s1
    let s1 = s;
    // owner of s is s2
    let s2 = s1;
    println!("{s2}");
    // this will not compile
    // print!("{s}");

    // 3
    let s = "Hello ".to_string();
    // if s goes to a function, looping, etc.
    take(s);
    // s dropped
    // println!("{s}");

    // Borrow
    // temporarily use a value without
    // taking ownership
    // 1. Creates a reference (either mutable or immutable)
    let s = "rust".to_string();
    let s1 = &s;
    let s2 = &s;
    let s3 = s2;
    // mutable borrow

    // 2. Doesn't move ownership
    // 3. Immutable reference - any number of read-only access to a value
    // 4. Mutable reference - only one read and write access to a value at a time
    let mut s = "rust".to_string();
    let s1 = &mut s;
    s1.push_str("🦀");
    let s2 = &mut s;
    s2.push_str("🦀");
    println!("{s2}");
    println!("{s}");

    // 5. Either immutable or mutable borrow, but not both simultaneously
    let s = "rust".to_string();
    let s1 = &s;
    let s2 = &s;
    // will not compile cuz there are mut and immut at a time
    // let s3 = &mut s;

    // 6. Reference must not outlive the value
    let s = "rust".to_string();
    let s1 = &s;
    {
        let s2 = s;
    }
    // s2 and s no longer exist
    // s1 references s
}

fn take(s: String) {
    println!("take {s}");
}