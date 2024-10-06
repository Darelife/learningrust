#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn get_sum(x: i32, y: i32) -> (i32, i32, i32) {
    // x + y
    // or
    return (x, y, x + y);
}

fn get_vector_sum(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += val;
    }
    return sum;
}
fn main() {
    let (a, b, c) = get_sum(1, 2);
    println!(
        "{} + {} = {}",
        get_sum(1, 2).0,
        get_sum(1, 2).1,
        get_sum(1, 2).2
    );

    let v = vec![1, 2, 3, 4, 5];
    println!("Sum of vector: {}", get_vector_sum(&v));
}
