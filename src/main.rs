extern crate rary;
use rary::english::hello;

mod mod_example;
use mod_example::*;

fn main() {
    println!("{}", hello());

    mod_example();
}
