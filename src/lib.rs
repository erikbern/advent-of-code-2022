use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_input() -> Vec<String> {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).expect("Can't open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}