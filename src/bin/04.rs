use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Interval {
    lo: i32,
    hi: i32,
}

fn parse_interval(s: &String) -> Interval {
  let v: Vec<i32> = s.split("-").map(|v| v.parse().unwrap()).collect();
  Interval { lo: v[0], hi: v[1] }
}

fn parse_line(s: &String) -> (Interval, Interval) {
  let v: Vec<String> = s.split(",").map(|v| v.parse().unwrap()).collect();
  (parse_interval(&v[0]), parse_interval(&v[1]))
}

fn main() {
  let path = env::args().nth(1).unwrap();
  let file = File::open(path).expect("Can't open file");
  let reader = BufReader::new(file);
  let (mut count_1, mut count_2) = (0u32, 0u32);
  for line in reader.lines() {
    let line = line.unwrap();
    let (p, q) = parse_line(&line);
    if (p.lo <= q.lo && p.hi >= q.hi) || (q.lo <= p.lo && q.hi >= p.hi) {
      count_1 += 1;
    }
    if ((p.lo + p.hi + 1) - (q.lo + q.hi + 1)).abs() < (p.hi - p.lo + q.hi - q.lo + 2) {
      count_2 += 1
    }
  }
  println!("{} {}", count_1, count_2);
}