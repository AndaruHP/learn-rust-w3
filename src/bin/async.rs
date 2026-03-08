#![allow(unused)]

use std::time::Duration;

struct Tomato;
struct Lettuce;
struct Cheese;
struct Patty;
struct Bun;

struct Hamburger {
    pub tomato: Tomato,
    pub lettuce: Lettuce,
    pub cheese: Cheese,
    pub patty: Patty,
    pub bun: Bun
}

// fn toast_bun() -> Bun {
//     Bun
// }
//
// fn cook_patty() -> Patty {
//     Patty
// }
//
// fn get_veggies() -> (Tomato, Lettuce) {
//     (Tomato, Lettuce)
// }
//
// fn get_cheese() -> Cheese {
//     Cheese
// }
//
// fn make_hamburger_seq() -> Hamburger {
//     let bun = toast_bun();
//     let patty = cook_patty();
//     let (tomato, lettuce) = get_veggies();
//     let cheese = get_cheese();
//
//     print!("Hamburger is ready");
//
//     Hamburger {
//         tomato,
//         lettuce,
//         cheese,
//         patty,
//         bun
//     }
// }

// let's make it async

async fn toast_bun() -> Bun {
    Bun
}

async fn cook_patty() -> Patty {
    Patty
}

async fn get_veggies() -> (Tomato, Lettuce) {
    (Tomato, Lettuce)
}

async fn get_cheese() -> Cheese {
    Cheese
}

async fn make_hamburger_seq() -> Hamburger {
    let bun = toast_bun().await;
    let patty = cook_patty().await;
    let (tomato, lettuce) = get_veggies().await;
    let cheese = get_cheese().await;

    print!("Hamburger is ready");

    Hamburger {
        tomato,
        lettuce,
        cheese,
        patty,
        bun
    }
}

async fn make_hamburger() -> Hamburger {
    let (bun, patty, (tomato, lettuce), cheese) = tokio::join!(
        toast_bun(),
        cook_patty(),
        get_veggies(),
        get_cheese()
    );

    print!("Hamburger is ready");

    Hamburger {
        tomato,
        lettuce,
        cheese,
        patty,
        bun
    }
}

// when to use threads vs async / await
// need to parallelize computation -> threads
// need to parallelize waiting time -> async / await

#[tokio::main]
async fn main() {
    // make_hamburger_seq().await;
    let fut = make_hamburger();
    fut.await;

    // spawning too many threads can crash this program (OS thread and memory limits)
    // let mut handles = vec![];
    // for i in 0..1000000 {
    //     handles.push(std::thread::spawn(move || {
    //         std::thread::sleep(Duration::from_millis(100));
    //         print!("Hamburger {} is ready", i);
    //     }))
    // }
    //
    // for h in handles {
    //     h.join().unwrap();
    // }

    let mut handles = vec![];
    for i in 0..1000000 {
        let fut = async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            print!("Hamburger {} is ready", i);
        };

        let handler = tokio::task::spawn(fut);
        handles.push(handler);
    }

    for h in handles {
        h.await.unwrap();
    }
}