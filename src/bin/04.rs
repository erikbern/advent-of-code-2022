use aoc::get_input;
use sscanf;

#[derive(Debug)]
struct Interval {
    lo: i32,
    hi: i32,
}

fn parse_line(s: &String) -> (Interval, Interval) {
  let parsed = sscanf::sscanf!(s, "{}-{},{}-{}", i32, i32, i32, i32);
  let (a_lo, a_hi, b_lo, b_hi) = parsed.unwrap();
  (Interval { lo: a_lo, hi: a_hi}, Interval { lo: b_lo, hi: b_hi})
}

fn main() {
  let lines = get_input();
  let (mut count_1, mut count_2) = (0u32, 0u32);
  for line in lines {
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