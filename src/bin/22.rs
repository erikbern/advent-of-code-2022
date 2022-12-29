use aoc::get_input;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    turn: i32,  // 1 or -1
    steps: i32,
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
    println!("{} * {}", h, w);
    let regex = Regex::new(r"(\d+|[LR])").unwrap();

    let instructions: Vec<Instruction> = regex.find_iter(&path).map(|m| {
        let m = m.as_str();
        match m {
            "L" => Instruction { turn: -1, steps: 0 },
            "R" => Instruction { turn: 1, steps: 0 },
            _ => Instruction { turn: 0, steps: m.parse::<i32>().unwrap() },
        }
    }).collect();

    let x: i32 = (0..w).filter(|x| grid[0][*x] != ' ').next().unwrap() as i32;
    let mut pos: (i32, i32) = (0, x);

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];  // (dy, dx), RDLU
    let mut dir = 0i32;

    for i in &instructions {
        println!("{:?} {}: {:?}", pos, dir, i);
        if i.turn != 0 {
            dir = (dir + i.turn).rem_euclid(4);
        } else {
            for _ in 0..i.steps {
                let mut pos_2 = pos;
                for dist in 1.. {
                    pos_2 = (
                        (pos.0 + dirs[dir as usize].0 * dist).rem_euclid(h as i32),
                        (pos.1 + dirs[dir as usize].1 * dist).rem_euclid(w as i32),
                    );
                    if grid[pos_2.0 as usize][pos_2.1 as usize] != ' ' {
                        break;
                    }
                }
                let g: char = grid[pos_2.0 as usize][pos_2.1 as usize];
                if g == '#' {
                    break;
                } else if g == '.' {
                    pos = pos_2;
                } else {
                    panic!("error");
                }
            }
        }
        println!(" -> {:?} {}", pos, dir);
    }
    println!("{}", 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir);
}