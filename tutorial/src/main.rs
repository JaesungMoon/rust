#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.40
    };
    bob.address = String::from("123 Elm St");
}
