use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
   let path = env::args().nth(1).unwrap();
   let file = File::open(path).expect("Can't open file");
   let reader = BufReader::new(file);
   let mut score_a = 0u32;
   let mut score_b = 0u32;
   for line in reader.lines() {
      let chars: Vec<char> = line.unwrap().chars().collect();
      let p = chars[0] as u32 - 'A' as u32;
      let q = chars[2] as u32 - 'X' as u32;
      let s1 = 3 * ((4 + q - p) % 3);
      let s2 = 1 + q;
      score_a += s1 + s2;
      let q_b = (p + q + 2) % 3;
      let s1_b = 3 * q;
      let s2_b = 1 + q_b;
      score_b += s1_b + s2_b;
      // println!("{} {} {} {}", s1, s2, s1_b, s2_b);
   }
   println!("{} {}", score_a, score_b);
}