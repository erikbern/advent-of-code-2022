use aoc::get_input;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::read_to_string;
use std::env;

#[derive(Debug)]
struct Instruction {
    turn: i32,  // 1 or -1
    steps: i32,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    y: i32,
    x: i32,
    d: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cut {
    x: i32,
    y: i32,
    d: i32,
    x2: i32,
    y2: i32,
    d2: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cuts {
    grid_size: i32,
    cuts: Vec<Cut>,
}

fn main() {
    let mut input = get_input();
    let path = input.pop().unwrap();
    input.pop();
    let h: usize = input.len();
    let w: usize = input.iter().map(|line| line.len()).max().unwrap();
    let grid: Vec<Vec<char>> = input.iter().map(|line| {
        let mut line: Vec<char> = line.chars().collect();
        while line.len() < w {
            line.push(' ');
        }
        line
    }).collect();
    let regex = Regex::new(r"(\d+|[LR])").unwrap();

    let instructions: Vec<Instruction> = regex.find_iter(&path).map(|m| {
        let m = m.as_str();
        match m {
            "L" => Instruction { turn: -1, steps: 0 },
            "R" => Instruction { turn: 1, steps: 0 },
            _ => Instruction { turn: 0, steps: m.parse::<i32>().unwrap() },
        }
    }).collect();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];  // (dy, dx), RDLU

    // Read 3D cuts from JSON
    let path = env::args().nth(2).unwrap();
    let json_data = read_to_string(path).unwrap();
    let cuts: Cuts = serde_json::from_str(&json_data).unwrap();

    let x: i32 = (0..w).filter(|x| grid[0][*x] != ' ').next().unwrap() as i32;

    let step_1 = |pos: &Position| -> Position {
        Position {
            y: pos.y + dirs[pos.d as usize].0,
            x: pos.x + dirs[pos.d as usize].1,
            d: pos.d,
        }
    };

    let wrap_2d = |pos: &Position| -> Position {
        Position {
            y: pos.y.rem_euclid(h as i32),
            x: pos.x.rem_euclid(w as i32),
            d: pos.d,
        }
    };

    let get_grid = |pos: &Position| -> char {
        grid[pos.y as usize][pos.x as usize]
    };

    let walk_2d = |pos: &Position| -> Position {
        let mut pos_2 = *pos;
        for _d in 1.. {
            pos_2 = step_1(&pos_2);
            pos_2 = wrap_2d(&pos_2);
            if get_grid(&pos_2) != ' ' {
                break;
            }
        }
        pos_2
    };

    let walk_3d = |pos: &Position| -> Position {
        let pos_2 = step_1(pos);
        if
            0 <= pos_2.y && pos_2.y < h as i32
            && 0 <= pos_2.x && pos_2.x < w as i32
            && get_grid(&pos_2) != ' '
        {
            pos_2
        } else {
            let mut ym = pos.y.rem_euclid(cuts.grid_size);
            let mut xm = pos.x.rem_euclid(cuts.grid_size);
            let y = pos_2.y.div_euclid(cuts.grid_size);
            let x = pos_2.x.div_euclid(cuts.grid_size);
            let c: &Cut = cuts.cuts.iter().find(|c| {c.x == x && c.y == y && c.d == pos.d}).unwrap();
            for _i in 0..(c.d2 - c.d).rem_euclid(4) {  // rotate clockwise
                let xm2 = cuts.grid_size - 1 - ym;
                let ym2 = xm;
                ym = ym2;
                xm = xm2;
            }
            Position {
                y: c.y2 * cuts.grid_size + ym + dirs[c.d2 as usize].0,
                x: c.x2 * cuts.grid_size + xm + dirs[c.d2 as usize].1,
                d: c.d2,
            }
        }
    };

    for part in 0..2 {
        let mut pos = Position {y: 0, x: x, d: 0 };

        for i in &instructions {
            // println!("{:?}: {:?}", pos, i);
            if i.turn != 0 {
                pos.d = (pos.d + i.turn).rem_euclid(4);
            } else {
                for _ in 0..i.steps {
                    let pos_2 = if part == 1 {
                        walk_3d(&pos)
                    } else {
                        walk_2d(&pos)
                    };
                    let g = get_grid(&pos_2);
                    if g == '#' {
                        break;
                    } else if g == '.' {
                        pos = pos_2;
                    } else {
                        panic!("error");
                    }
                }
            }
            // println!(" -> {:?}", pos);
        }
        println!("{}", 1000 * (pos.y + 1) + 4 * (pos.x + 1) + pos.d);
    }
}