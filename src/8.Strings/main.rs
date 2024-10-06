#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" string");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st_random = String::from("x r t b b h k k m a c");
    let mut v1: Vec<char> = st_random.chars().collect();
    v1.sort();
    v1.dedup(); // removes duplicates
    for char in v1 {
        print!("{} ", char);
    }
    println!();
    let st4 = "Random String";
    let mut st5 = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("St6 Length : {}", st6.len());
    st5.clear();
    let st6 = String::from("Just Some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7; // st6 is moved here, while st7 is borrowed
    for char in st8.bytes() {
        println!("{}", char);
    }
}
