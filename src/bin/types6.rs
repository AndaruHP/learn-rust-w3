#![allow(unused)]

use::std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("red".to_string(), 100);
    scores.insert("blue".to_string(), 200);
    println!("{:#?}", scores);

    // get
    let score: Option<&u32> = scores.get("red");
    println!("Red: {:?}", score);
    let score: Option<&u32> = scores.get("yellow");
    println!("Yellow: {:?}", score);

    // update
    let score: &mut u32 = scores.entry("black".to_string()).or_insert(1000);
    *score += 1;
    println!("Black: {:?}", score);
}