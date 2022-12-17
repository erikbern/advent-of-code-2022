use aoc::get_input;

fn main() {
    let lines = get_input();
    let grid: Vec<&[u8]> = lines.iter().map(|s| s.as_bytes()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut visible = vec![vec![false; cols]; rows];
    let mut visible_ct = 0u32;
    let mut scenic_score = vec![vec![1u32; cols]; rows];

    let mut walk = |mut y: i32, mut x: i32, dy: i32, dx: i32| {
        let mut highest = 0u8;
        let mut stack = Vec::<(u8, u32)>::new();
        stack.push((255, 1));
        let mut steps = 1u32;
        while x >= 0 && y >= 0 && x < cols as i32 && y < rows as i32 {
            let height = grid[y as usize][x as usize];
            if height > highest {
                if !visible[y as usize][x as usize] {
                    visible[y as usize][x as usize] = true;
                    visible_ct += 1;
                }
                highest = height;
            }
            // Find the last one that's higher
            while stack.last().unwrap().0 < height {
                stack.pop();
            }
            let distance = steps - stack.last().unwrap().1;
            scenic_score[y as usize][x as usize] *= distance;
            stack.push((height, steps));
            x += dx;
            y += dy;
            steps += 1;
        }
    };
    
    for row in 0..rows {
        walk(row as i32, 0, 0, 1);
        walk(row as i32, cols as i32 - 1, 0, -1);
    }
    for col in 0..cols {
        walk(0, col as i32, 1, 0);
        walk(rows as i32 - 1, col as i32, -1, 0);
    }

    println!("{}", visible_ct);

    let mut max = 0u32;
    for row in 0..rows {
        for col in 0..cols {
            if scenic_score[row][col] > max {
                max = scenic_score[row][col];
            }
        }
    }
    println!("{}", max);
}