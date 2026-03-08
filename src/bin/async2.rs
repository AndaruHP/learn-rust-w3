#![allow(unused)]

use std::time::Duration;
use tokio::{join, select};

// join
// - polls multiple futures concurrently
// - waits for all of them complete
// - returns a tuple of their results

// select
// - polls multiple futures concurrently
// - returns as soon as one of them completes
// - cancels the rest (drops them)

// simulates an async task that completes after `dt` milliseconds
async fn make(val: &'static str, dt: u64) -> &'static str {
    tokio::time::sleep(Duration::from_millis(dt)).await;
    val
}

#[tokio::main]
async fn main() {
    let (res1, res2, res3) = join!(
        make("coffee", 100),
        make("green tea", 50),
        make("lemonade", 20),
    );
    println!("join: res1 = {:?}", res1);
    println!("join: res2 = {:?}", res2);
    println!("join: res3 = {:?}", res3);

    let res = select!(
        val = make("coffee", 100) => {
            println!("future 1 finished first");
        },
        val = make("green tea", 50) => {
            println!("future 2 finished first");
        },
        val = make("lemonade", 20) => {
            println!("future 3 finished first");
        }
    );

}