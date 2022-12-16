use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sscanf;

fn main() {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).expect("Can't open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let index: usize = lines.iter().position(|line| line.len() == 0).unwrap();
    let rows = index - 1;
    let cols = (lines[0].len() + 1)/4;
    let mut grid_a: Vec<Vec<u8>> = vec![Vec::new(); cols];
    let mut grid_b: Vec<Vec<u8>> = vec![Vec::new(); cols];
    for y in (0..rows).rev() {
        let row: &[u8] = lines[y].as_bytes();
        for x in 0..cols {
            let ch: u8 = row[x*4+1];
            if 'A' as u8 <= ch && ch <= 'Z' as u8 {
                grid_a[x].push(ch);
                grid_b[x].push(ch);
            }
        }
    }
    for line in &lines[index+1..] {
        let parsed = sscanf::sscanf!(line, "move {} from {} to {}", usize, usize, usize);
        let (n, src, dst) = parsed.unwrap();
        for _i in 0..n {
            let ch: u8 = grid_a[src-1].pop().unwrap();
            grid_a[dst-1].push(ch);
        }
        let mut tmp = Vec::<u8>::new();
        for _i in 0..n {
            let ch: u8 = grid_b[src-1].pop().unwrap();
            tmp.push(ch);
        }
        for _i in 0..n {
            let ch = tmp.pop().unwrap();
            grid_b[dst-1].push(ch);
        }
    }

    let top_a: Vec<u8> = grid_a.into_iter().map(|v| *v.last().unwrap()).collect();
    let top_a_str = std::str::from_utf8(&top_a).unwrap();
    println!("{}", top_a_str);

    let top_b: Vec<u8> = grid_b.into_iter().map(|v| *v.last().unwrap()).collect();
    let top_b_str = std::str::from_utf8(&top_b).unwrap();
    println!("{}", top_b_str);

}