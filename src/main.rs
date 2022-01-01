use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Lines};

pub fn file_to_lines(file_path: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(file_path)?;
    let b = BufReader::new(f);
    let lines = b.lines();
    Ok(lines)
}

fn main() {
    println!("Hello!");
    for line in file_to_lines("data/example.txt") {
        println!("{:?}", line);
    }
}
