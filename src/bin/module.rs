use apacona::my_module::{foo, my};

fn main() {
    my::public_print();
    my::a::public_print();

    let s = my::a::build(1, "Alice".to_string());
    println!("s = {:?}", s);

    foo::public_print();
}
