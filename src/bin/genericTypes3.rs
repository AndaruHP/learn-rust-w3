#![allow(unused)]

use std::path::Component::ParentDir;

struct Solidity {
    version: String
}

struct Vyper {
    version: String
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
    fn help(&self) -> String {
        "Good Luck!".to_string()
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {}", file_path)
    }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8".to_string()
    };
    let vy = Vyper {
        version: "0.1".to_string()
    };

    println!("sol compile: {}", sol.help());
    println!("vyper compile: {}", vy.help());

    println!("sol compile: {}", sol.compile("hello.sol"));
    println!("vyper compile: {}", vy.compile("hello.vy"));

    println!("sol compile: {}", compile(&sol, "hello.sol"));
    println!("vyper compile: {}", compile(&vy, "hello.vy"));

}