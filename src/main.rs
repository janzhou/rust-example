extern crate rary;
use rary::english::hello;

mod mod_example;
use mod_example::*;

mod thread_example;
use thread_example::*;

mod borrow;
use borrow::*;

use std::iter::Sum;
use std::ops::Mul;

fn dot_product<T>(a: Vec<T>, b: Vec<T>) -> T
where T: Sum + Copy + Mul<Output = T>
{
    return a.iter().zip(b.iter()).map(|(&x, &y)| x*y).sum();
}

fn main() {
    println!("{}", hello());

    println!("dot product: {}", dot_product(vec![1], vec![2]));
    mod_example();
    borrow();
    thread_example();
}
