use aoc::get_input;
use std::collections::{HashMap, HashSet};

type YX = (i32, i32);

fn main() {
    let mut elves = Vec::<YX>::new();
    for (y, line) in get_input().iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                let yx: YX = (y as i32, x as i32);
                elves.push(yx);
            }
        }
    }
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut nns = Vec::<YX>::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if !(dx == 0 && dy == 0) {
                nns.push((dy, dx));
            }
        }
    }
    let go = |pos: &YX, d: usize| -> YX {
        (pos.0 + dirs[d].0, pos.1 + dirs[d].1)
    };
    for step in 0.. {
        // Compute distinct positions
        let m = HashSet::<YX>::from_iter(elves.iter().cloned());

        // Propose directions
        let p: Vec<YX> = elves.iter().map(|yx| {
            if nns.iter()
                .map(|dyx| (yx.0 + dyx.0, yx.1 + dyx.1))
                .all(|yx2| !m.contains(&yx2))
            {
                return *yx;
            }
            for d in 0..4 {
                let dir = (d + step) % 4;
                let spots = vec![go(&yx, dir), go(&go(&yx, dir), dir^2), go(&go(&yx, dir), dir^3)];
                if spots.iter().all(|yx2| !m.contains(&yx2)) {
                    return go(&yx, dir);
                }
            }
            return *yx;
        }).collect();

        // Compute distinct proposed positions
        let mut c = HashMap::<YX, usize>::new();
        for yx in &p {
            *c.entry(*yx).or_insert(0) += 1;
        }

        // Compute final destinations, removing collisions
        let new_elves: Vec<YX> = elves.iter().zip(p.iter()).map(|(yx, yx2)| {
            if c.get(&yx2).unwrap_or(&0) <= &1 { *yx2 } else { *yx }
        }).collect();

        // See if any elf moved
        let moved = elves.iter().zip(new_elves.iter()).any(|(yx, yx2)| yx != yx2);
        if !moved {
            println!("Didn't move on round {}", step+1);
        }

        // Move all elves
        elves = new_elves;
 
        if step == 9 {
            let y_min = elves.iter().map(|yx| yx.0).min().unwrap();
            let y_max = elves.iter().map(|yx| yx.0).max().unwrap();
            let x_min = elves.iter().map(|yx| yx.1).min().unwrap();
            let x_max = elves.iter().map(|yx| yx.1).max().unwrap();
            let res = (y_max - y_min + 1) * (x_max - x_min + 1) - (elves.len() as i32);
            println!("after 10 steps: ({} - {} + 1) * ({} - {} + 1) - {} = {}", y_max, y_min, x_max, x_min, elves.len(), res);
        }
        if step > 9 && !moved {
            break;
        }
    }
}