#![allow(unused)]

pub mod my {
    pub fn public_print() {
        println!("public")
    }

    fn private_print() {
        println!("private")
    }

    pub mod a {
        pub fn public_print() {
            println!("a")
        }

        #[derive(Debug)]
        pub struct S {
            id: i32,
            name: String,
        }

        pub fn build(id: i32, name: String) -> S {
            S { id, name }
        }
    }
}

pub mod foo {
    use super::my;

    pub fn public_print() {
        my::public_print();
    }
}
