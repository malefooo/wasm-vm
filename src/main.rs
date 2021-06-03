#![feature(array_methods)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


mod binary;
mod common;
mod interpreter;
mod utils;

fn main() {
    binary::init();
    println!("Hello, world!");
}
