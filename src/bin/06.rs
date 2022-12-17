use aoc::get_input;
use std::collections::HashSet;

fn find_distinct(line: &[u8], n: usize) -> Option<usize> {
    let mut first: Option<usize> = None;
    for i in 0..line.len()-n+1 {
        let mut s = HashSet::<u8>::new();
        for j in i..i+n {
            s.insert(line[j]);
        }
        if s.len() == n {
            first = Some(i+n);
            break;
        }
    }
    first
}

fn main() {
    let lines = get_input();
    for line in lines {
        let line = line.as_bytes();
        let first4 = find_distinct(line, 4);
        let first14 = find_distinct(line, 14);
        println!("{} {}", first4.unwrap(), first14.unwrap());
    }
}