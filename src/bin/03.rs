use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn get_set(s: &str) -> HashSet<char> {
  HashSet::from_iter(s.chars())
}

fn get_score(ch: &char) -> u32 {
  match ch {
    'a'..='z' => 1 + *ch as u32 - 'a' as u32,
    'A'..='Z' => 27 + *ch as u32 - 'A' as u32,
    _ => 0,
  }
}

fn main() {
   let path = env::args().nth(1).unwrap();
   let file = File::open(path).expect("Can't open file");
   let reader = BufReader::new(file);
   let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
   let mut sum_1 = 0u32;
   for line in &lines {
     let n = line.len();
     let p = get_set(&line[..n/2]);
     let q = get_set(&line[n/2..]);
     sum_1 += p.intersection(&q).map(get_score).sum::<u32>();
   }
   println!("{}", sum_1);
   let mut sum_2 = 0u32;
   for i in 0..lines.len()/3 {
     let p = get_set(&lines[3*i]);
     let q = get_set(&lines[3*i + 1]);
     let r = get_set(&lines[3*i + 2]);
     let pq: HashSet<char> = HashSet::from_iter(p.intersection(&q).map(|ch| *ch));
     sum_2 += pq.intersection(&r).map(get_score).sum::<u32>();
   }
   println!("{}", sum_2);
}