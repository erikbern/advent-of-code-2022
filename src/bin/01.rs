use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
   let args: Vec<String> = env::args().collect();
   let path = &args[1];
   let file = File::open(path).expect("Can't open file");
   let reader = BufReader::new(file);
   let mut sum: i32 = 0;
   let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
   let mut elves = Vec::<i32>::new();
   for line in lines {
     if line == "" {
       elves.push(sum);
       sum = 0;
     } else {
       sum += line.parse::<i32>().unwrap();
     }
   }
   elves.sort();
   println!("{}", elves.last().unwrap());
   println!("{}", elves[elves.len()-3..].iter().sum::<i32>());
}
