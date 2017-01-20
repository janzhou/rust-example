extern crate rary;
use rary::english::hello;

mod mod_example;
use mod_example::*;

mod thread_example;
use thread_example::*;

mod borrow;
use borrow::*;

mod vector;
use vector::*;

fn main() {
    println!("{}", hello());

    println!("dot product: {}", dot_product(vec![1], vec![2]));

    let a = Vector::new(vec![1]);
    let b = Vector::new(vec![2]);
    println!("dot product: {}", a * b);

    mod_example();
    borrow();
    thread_example();
}
