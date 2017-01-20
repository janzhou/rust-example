extern crate rary;
use rary::english::hello;

mod mod_example;
use mod_example::*;

mod thread_example;
use thread_example::*;

mod borrow;
use borrow::*;

mod dot;
use dot::*;

fn main() {
    println!("{}", hello());

    println!("dot product: {}", dot_product(vec![1], vec![2]));
    dot();
    mod_example();
    borrow();
    thread_example();
}
