#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 4,
        height: 1.5
    };
}
