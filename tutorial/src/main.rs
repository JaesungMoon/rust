#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut heros = HashMap::new();
    heros.insert("Superman", "Clark Kent");
    heros.insert("Batman", "Bruce Wayne");
    heros.insert("The Flash", "Barry Allen");

    for (k, v) in heros.iter() {
        println!("{} = {}", k, v);
    }
    if heros.contains_key(&"Superman") {
        let the_batman = heros.get(&"Superman");
        match the_batman {
            Some(x) => println!("The Batman is a hero"),
            None => println!("The Batman is not a hero"),
        }
    }
}
