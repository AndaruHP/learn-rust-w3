#![allow(unused)]

fn main() {
    // signed integers
    // range -(2^(n-1) to 2^(n-1) - 1
    // -(2^(8-1) to 2^(8-1) - 1
    let i0: i8 = -1;
    // -(2^(16-1) to 2^(16-1) - 1
    let i1: i16 = -1;
    // -(2^(32-1) to 2^(32-1) - 1
    let i2: i32 = -1;
    // etc

    // unsigned integers
    // range 0 to 2^n - 1
    // 2^8 - 1
    let i3: u8 = 1;

    // depends on computer architecture
    let i4: isize = -1;
    let i5: usize = 1;

    // float
    let f0: f32 = 0.01;
    let f1: f64 = 0.01;

    // boolean
    let b: bool = false;

    // character
    let c: char = 'c';

    // type conversion
    let i: i32 = -1;
    let j: u32 = i as u32;

    // min max
    let i_max: i32 = i32::MAX;
    let u_min: i32 = i32::MIN;

    // overflow
    let mut x = u32::MAX;
    // x += 1; // it will return 1
    // u32::checked_add -> return None on overflow
    let x = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", x); // None
    // u32::wrapping_add -> explicitly allow overflow
    let x = u32::wrapping_add(u32::MAX, 1);
    println!("checked_add: {:?}", x); // 1

    // tuple
    let t: (bool, char, u32) = (true, 'c', u32::MAX);
    println!("{} {} {}", t.0, t.1, t.2);

    let t = (); // empty tuple = unit type
    let nested = (('a', 1.23), (true, 1u32, 90i32), ());
    println!("{}", nested.0.0);

    // destructuring a tuple
    let t: (bool, char, u32) = (true, 'c', u32::MAX);
    let (a, b, c) = t;
    println!("a = {}, b = {}, c = {}", a, b, c);
    let (_, z, _) = t;
    println!("z = {}", z);

    // array
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);
    let mut arr: [u32; 3] = [1, 2, 3];
    arr[0] = 1;
    let arr: [u32; 10] = [0; 10];
    println!("arr[0] = {:?}", arr);
    // slicing
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
    // first 3
    let s: &[i32] = &nums[..3];
    // last 3
    let s: &[i32] = &nums[3..];
    // middle 4
    let s: &[i32] = &nums[3..7];

    
}