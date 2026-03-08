#![allow(unused)]

use std::collections::HashMap;

// iterators
fn main() {
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    // into_iter -> iterate T
    // iter -> iterate &T
    // iter_mut -> iterate &mut T

    // for v in vals.into_iter() {}
    // for v in vals.iter() {}
    // for v in vals.into_mut() {}

    // map
    let v2: Vec<(u32)> = vals.iter().map(|x: &u32| x + 1).collect();
    println!("v2 {:?}", v2);

    // collect
    let vals: Vec<(&str, u32)> = vec![("a", 1), ("b", 2), ("c", 3), ("d", 4)];
    let v: Vec<(String, u32)> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("v {:?}", v);
    let v: HashMap<String, u32> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("v {:?}", v);

    // chaining filter and map
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals.iter().filter(|x: &&u32| **x <= 3).map(|x: &u32| x + 1).collect();
    println!("v {:?}", v);

    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals.into_iter().filter(|x: &u32| *x <= 3).map(|x: u32| x + 1).collect();
    println!("v {:?}", v);

}