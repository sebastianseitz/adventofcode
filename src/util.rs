use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

use std::io::Error;


pub fn read_lines(filename: String) -> Vec<String> {
    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    let result: Result<Vec<String>, Error> = file.lines().collect();
    return result.unwrap();
}
