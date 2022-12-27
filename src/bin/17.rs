use aoc::get_input;
use std::collections::HashSet;

fn main() {
    let pieces = vec![  // (x, y) where y goes up
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let jet_pattern: Vec<char> = get_input()[0].chars().collect();
    let mut jet_pattern_index = 0;
    let mut blocks = HashSet::<(i32, i32)>::new();
    let mut hist_step = vec![vec![0; pieces.len()]; jet_pattern.len()];
    let mut hist_block_max_y = vec![vec![0i32; pieces.len()]; jet_pattern.len()];
    let mut block_max_y = -1;
    let n: i64 = 1000000000000;
    for step in 0..10000 {
        let piece_index = step % pieces.len();
        let piece = &pieces[piece_index];
        /*
        for y in (0..=block_max_y).rev() {
            let row: String = (0..7).map(|x| if blocks.contains(&(x, y)) { '#' } else { '.'}).collect();
            println!("{}", row);
        }
        */
        let mut piece_y = block_max_y + 4;
        let mut piece_x = 2;
        loop {
            let jet = jet_pattern[jet_pattern_index];
            jet_pattern_index = (jet_pattern_index + 1) % jet_pattern.len();
            let dx: i32 = match jet {
                '<' => -1,
                '>' => 1,
                _ => panic!(),
            };
            // Try to move sideway
            if piece.iter().map(|(x, y)| (x + dx + piece_x, y + piece_y)).all(|(x, y)| {
                x >= 0 && x < 7 && !blocks.contains(&(x, y))
            }) {
                piece_x += dx;
            }
            // Try to move down
            if piece.iter().map(|(x, y)| (x + piece_x, y + piece_y - 1)).all(|(x, y)| {
                y >= 0 && !blocks.contains(&(x, y))
            }) {
                piece_y -= 1;
            } else {
                break;
            }
        }
        // Write blocks
        for (x, y) in piece {
            blocks.insert((x + piece_x, y + piece_y));
        }
        block_max_y = *blocks.iter().map(|(_x, y)| y).max().unwrap();
        if step == 2022 - 1 {
            println!("{}", block_max_y + 1);
        }
        let step_0 = hist_step[jet_pattern_index][piece_index];
        let block_max_y_0 = hist_block_max_y[jet_pattern_index][piece_index];
        hist_step[jet_pattern_index][piece_index] = step;
        hist_block_max_y[jet_pattern_index][piece_index] = block_max_y;
        if step_0 > 0 {
            let period = (step - step_0) as i64;
            if (n - 1) % period == (step as i64) % period {
                let periods: i64 = ((n - 1) - step as i64) / period;
                let period_blocks: i64 = (block_max_y - block_max_y_0) as i64;
                println!("{}", (block_max_y as i64) + period_blocks * periods + 1);
            }
        }
    }
}