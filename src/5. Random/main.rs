#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}
