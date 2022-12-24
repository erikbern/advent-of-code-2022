use aoc::get_input;
use std::collections::HashSet;

fn parse_coords(s: &str) -> (i32, i32) {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}


fn main() {
    let lines = get_input();
    let mut h = HashSet::<(i32, i32)>::new();
    for line in lines {
        let wall: Vec<(i32, i32)> = line.split(" -> ").map(parse_coords).collect();
        for j in 0..wall.len()-1 {
            let dx = wall[j+1].0 - wall[j].0;
            let dy = wall[j+1].1 - wall[j].1;
            let l = dx.abs() + dy.abs();
            for z in 0..=l {
                let x = wall[j].0 + z * dx / l;
                let y = wall[j].1 + z * dy / l;
                h.insert((x, y));
            }
        }
    }
    let mut steps = 0u32;
    'outer: loop {
        let mut x = 500;
        'inner: for y in 1.. {
            if y >= 1000 {
                break 'outer;
            } if !h.contains(&(x, y+1)) {
            } else if !h.contains(&(x-1, y+1)) {
                x = x - 1;
            } else if !h.contains(&(x+1, y+1)) {
                x = x + 1;
            } else {
                h.insert((x, y));
                break 'inner;
            }
        }
        steps += 1;
    }
    println!("{}", steps);
}