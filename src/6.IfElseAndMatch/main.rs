#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age = 8;
    if (age >= 1 && age <= 18) {
        println!("Important Birthday");
    }

    // Ternary operator
    let is_birthday: bool = if age >= 1 && age <= 18 { true } else { false };

    // match
    let age2 = 18;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 51 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an important birthday"),
    };

    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Not old enough to vote"),
        Ordering::Equal => println!("You've gained the right to vote"),
        Ordering::Greater => println!("Old enough to vote"),
    }
}
