use aoc::get_input;
use std::collections::HashSet;
use sscanf::sscanf;

fn main() {
    let mut h = HashSet::<(i32, i32, i32)>::new();
    for line in get_input() {
        let (x, y, z) = sscanf!(line, "{i32},{i32},{i32}").unwrap();
        h.insert((x, y, z));
    }
    let dirs = vec![(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

    // Part 1
    let mut surface = 0;
    for (x, y, z) in &h {
        surface += dirs.iter().filter(|(dx, dy, dz)| !h.contains(&(x + dx, y + dy, z + dz))).count();
    }
    println!("{}", surface);

    // Part 2
    let lo = -1i32;
    let hi = 20i32;
    let mut v = HashSet::<(i32, i32, i32)>::new();
    let mut q = Vec::<(i32, i32, i32)>::new();
    q.push((lo, lo, lo));
    let mut surface = 0;
    while q.len() > 0 {
        let (x, y, z) = q.pop().unwrap();
        for (dx, dy, dz) in &dirs {
            let (x2, y2, z2) = (x + dx, y + dy, z + dz);
            if !v.contains(&(x2, y2, z2)) {
                if x2 >= lo && x2 <= hi && y2 >= lo && y2 <= hi && z2 >= lo && z2 <= hi {
                    if h.contains(&(x2, y2, z2)) {
                        surface += 1;
                    } else {
                        v.insert((x2, y2, z2));
                        q.push((x2, y2, z2));
                    }
                }
            }
        }
    }
    println!("{}", surface);
}