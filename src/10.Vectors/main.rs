#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd element"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vector: {:?}", vec2);
    println!("Vector Length: {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}
