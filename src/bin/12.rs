use aoc::get_input;
use std::collections::{HashMap, HashSet};

fn main() {
    let lines = get_input();
    let w = lines[0].len() as i32;
    let h = lines.len() as i32;
    let mut start: Option<(i32, i32)> = None;
    let mut end: Option<(i32, i32)> = None;
    let lines: Vec<Vec<char>> = lines.iter().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, ch)| match ch {
            'S' => {end = Some((y as i32, x as i32)); 'a'},
            'E' => {start = Some((y as i32, x as i32)); 'z'},
            _ => ch,
        }).collect()
    }).collect();
    let mut visited = HashMap::<(i32, i32), i32>::new();
    visited.insert(start.unwrap(), 0);
    let mut cur = HashSet::<(i32, i32)>::new();
    cur.insert(start.unwrap());
    let mut step = 0i32;
    let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    while cur.len() > 0 {
        let next = HashSet::<(i32, i32)>::from_iter(
            cur
            .iter()
            .flat_map(|yx| dirs.iter().map(|dyx| (*yx, (yx.0 + dyx.0, yx.1 + dyx.1))))
            .filter(|(_yx, yx2)| yx2.0 >= 0 && yx2.0 < h && yx2.1 >= 0 && yx2.1 < w)
            .filter(|(yx, yx2)| {
                let z = lines[yx.0 as usize][yx.1 as usize] as i32;
                let z2 = lines[yx2.0 as usize][yx2.1 as usize] as i32;
                z <= z2 + 1
            })
            .map(|(_yx, yx2)| yx2)
            .filter(|yx2| !visited.contains_key(yx2))
        );
        next.iter().for_each(|yx| {
            visited.insert(*yx, step+1);
        });
        cur = next;
        step += 1;
    }
    println!("{}", visited[&end.unwrap()]);
    let shortest_a = visited
        .iter()
        .filter(|((y, x), _d)| lines[*y as usize][*x as usize] == 'a')
        .map(|(_yx, d)| *d)
        .min();
    println!("{}", shortest_a.unwrap());
}