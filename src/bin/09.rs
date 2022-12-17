use aoc::get_input;
use sscanf;
use std::cmp;
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn simulate(lines: &Vec<String>, n: usize) -> usize {
    let mut rope = Vec::<Point>::new();
    for _i in 0..n {
        rope.push(Point { x: 0, y: 0});
    }
    let mut visited = HashSet::<Point>::new();
    visited.insert(Point { x: 0, y: 0 });

    for line in lines {
        let parsed = sscanf::sscanf!(line, "{} {}", char, u32);
        let (dir, n_steps) = parsed.unwrap();
        for _i in 0..n_steps {
            // Update head position
            let (dx, dy) = match dir {
                'U' => (0, -1),
                'D' => (0, 1),
                'R' => (1, 0),
                'L' => (-1, 0),
                _ => (0, 0),
            };
            let head = &rope[0];
            rope[0] = Point { x: head.x + dx, y: head.y + dy};
            for i in 1..n {
                let head = &rope[i-1];
                let tail = &rope[i];
                if cmp::max((head.x - tail.x).abs(), (head.y - tail.y).abs()) > 1 {
                    rope[i] = Point {
                        x: tail.x + (head.x - tail.x).signum(),
                        y: tail.y + (head.y - tail.y).signum(),
                    }
                }
            }
            let tail = &rope[n-1];
            visited.insert(tail.clone());
        }
    }
    visited.len()
}

fn main() {
    let lines = get_input();
    println!("{}", simulate(&lines, 2));
    println!("{}", simulate(&lines, 10));
}