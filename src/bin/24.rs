use aoc::get_input;
use std::collections::HashSet;

type YX = (i32, i32);

#[derive(Debug)]
struct Blizzard {
    yx: YX,
    dyx: YX,
}

fn get_blocked(blizzards: &Vec<Blizzard>, step: i32, h: i32, w: i32) -> HashSet<YX> {
    let mut blocked = HashSet::<YX>::new();
    for blizzard in blizzards {
        let y = (blizzard.yx.0 + blizzard.dyx.0 * step - 1).rem_euclid(h) + 1;
        let x = (blizzard.yx.1 + blizzard.dyx.1 * step - 1).rem_euclid(w) + 1;
        blocked.insert((y, x));
    }
    blocked
}

fn get_attainable(prev_attainable: &HashSet<YX>, blocked: &HashSet<YX>, h: i32, w: i32) -> HashSet<YX> {
    let mut next_attainable = HashSet::<YX>::new();
    let dirs = vec![(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];
    let inside_rect = |yx: YX| yx.0 >= 1 && yx.0 <= h && yx.1 >= 1 && yx.1 <= w;
    let is_enter = |yx: YX| yx.0 == 0 && yx.1 == 1;
    let is_exit = |yx: YX| yx.0 == h+1 && yx.1 == w;
    let is_ok = |yx: YX| inside_rect(yx) || is_enter(yx) || is_exit(yx);    
    for yx in prev_attainable {
        for dir in &dirs {
            let yx2: YX = (yx.0 + dir.0, yx.1 + dir.1);
            if is_ok(yx2) && !blocked.contains(&yx2) {
                next_attainable.insert(yx2);
            }
        }
    }
    next_attainable
}

fn go(start: YX, goal: YX, blizzards: &Vec<Blizzard>, start_step: usize, h: i32, w: i32) -> usize {
    let mut attainable = HashSet::<YX>::new();
    attainable.insert(start);
    let mut step = start_step;
    while !attainable.contains(&goal) {
        step += 1;
        let blocked = get_blocked(&blizzards, step as i32, h, w);
        attainable = get_attainable(&attainable, &blocked, h, w);
    }
    step
}

fn main() {
    let input = get_input();
    let h: i32 = input.len() as i32 - 2;
    let w: i32 = input[0].len() as i32 - 2;
    let mut blizzards = Vec::<Blizzard>::new();
    for (y, line) in get_input().iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let yx: YX = (y as i32, x as i32);
            let dyx: Option<YX> = match ch {
                '^' => Some((-1, 0)),
                'v' => Some((1, 0)),
                '<' => Some((0, -1)),
                '>' => Some((0, 1)),
                _ => None,
            };
            match dyx {
                Some(dyx) => blizzards.push(Blizzard { yx, dyx }),
                _ => {}
            }
        }
    }
    let start: YX = (0, 1);
    let goal: YX = (h + 1, w);
    let step = go(start, goal, &blizzards, 0, h, w);
    println!("-> {}", step);
    let step = go(goal, start, &blizzards, step, h, w);
    println!("-> {}", step);
    let step = go(start, goal, &blizzards, step, h, w);
    println!("-> {}", step);
}