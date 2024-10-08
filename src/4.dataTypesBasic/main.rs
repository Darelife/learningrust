#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

struct Person<'a> {
    name: &'a str,
    age: u8,
}
fn main() {
    // Unsigned integers : u8, u16, u32, u64, u128, usize (depends on the architecture)
    // Signed integers : i8, i16, i32, i64, i128, isize (depends on the architecture)
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // if you have a variable which you aren't using, and you don't want to get a warning
    // you can prefix it with an underscore (eg: _unused_variable, instead of unused_variable)
    let _unused_variable = 10;

    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int_u32: u32 = int_u8 as u32 + (int2_u8) as u32;
}
