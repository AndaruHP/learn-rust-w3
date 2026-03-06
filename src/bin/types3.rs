#![allow(unused)]

use std::io::Error;

#[derive(Debug)]
#[derive(PartialEq)]
// Enum
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize {width: u32, height: u32}
}

fn main() {
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize {width: 1, height: 1};
    // Debug
    println!("{:?}", cmd);

    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("{}", cmd0 == cmd1);

    // Option<T> = Some(T) | None
    let x: Option<i32> = Some(1);
    let y: Option<i32> = None;

    // Result<T, E> = Ok(T) | Error(E)
    // "100" -> 100
    // "123azx" -> error
    let x: Result<i32, &str> = Ok(100);
    let x: Result<i32, &str> = Err("This is error");
    println!("{:?}", x);
}