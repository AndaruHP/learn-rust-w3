#![allow(unused)]

trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, bool, char) {
    fn count(&self) -> usize {3}
    fn first(&self) -> &u32 {
        &self.0
    }
}

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {3}
    fn first(&self) -> &T {
        &self[0]
    }
}

fn main() {
    let t = (1u32, true, 'c');
    println!("t.count(): {}", t.count());
    println!("t.first(): {}", t.first());

    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    println!("v.count(): {}", v.count());
    println!("v.first(): {}", v.first());
}