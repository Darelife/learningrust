#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_1 = [1, 2, 3, 4, 5];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());
    let mut loop_idx = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 5 {
            break;
        }
        println!("Odd number : {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    loop_idx = 0;
    while loop_idx < arr_1.len() {
        println!("While Element : {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_1.iter() {
        println!("For Element : {}", val);
    }

    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}
