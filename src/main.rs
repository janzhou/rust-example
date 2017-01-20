extern crate rary;
use rary::english::hello;

mod mod_example;
use mod_example::*;

mod thread_example;
use thread_example::*;

mod borrow;
use borrow::*;

fn main() {
    println!("{}", hello());

    mod_example();
    borrow();
    thread_example();
}
