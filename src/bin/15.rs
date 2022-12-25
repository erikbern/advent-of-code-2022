use std::env;
use aoc::get_input;
use sscanf::sscanf;
use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    r: i32,
}

fn main() {
    let lines = get_input();
    let args: Vec<i32> = env::args().skip(2).map(|s| s.parse::<i32>().unwrap()).collect();
    let (row_y, max_x, max_y) = (args[0], args[1], args[2]);
    let mut sensors = Vec::<Sensor>::new();
    let mut beacons_in_row = 0;
    for line in lines {
        let parsed = sscanf!(line, "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}");
        let (sx, sy, bx, by) = parsed.unwrap();
        sensors.push(Sensor {x: sx, y: sy, r: (bx - sx).abs() + (by - sy).abs()});
        if by == row_y {
            beacons_in_row += 1;
        }
    }
    let mut taken = HashSet::<i32>::new();
    for sensor in &sensors {
        let dy = (sensor.y - row_y).abs();
        if sensor.r >= dy {
            let x_lo = sensor.x - (sensor.r - dy);
            let x_hi = sensor.x + (sensor.r - dy);
            for x in x_lo..=x_hi {
                taken.insert(x);
            }
        }
    }
    println!("{}", taken.len() - beacons_in_row);
    'outer: for y in 0..=max_y {
        let mut row = Vec:: <(i32, i32)>::new();
        for sensor in &sensors {
            let dy = (sensor.y - y).abs();
            if sensor.r >= dy {
                let x_lo = sensor.x - (sensor.r - dy);
                let x_hi = sensor.x + (sensor.r - dy);
                row.push((x_lo, x_hi));
            }
        }
        row.sort();
        let mut last_x = 0i32;
        for (x_lo, x_hi) in row {
            if x_lo > last_x + 1 && last_x + 1 >= 0 && last_x + 1 <= max_x {
                println!("gap on row {}: {} - {}", y, last_x, x_lo);
                println!("tuning frequency: {}", 4000000 * (last_x as i64 + 1) + y as i64);
                break 'outer;
            }
            if x_hi > last_x {
                last_x = x_hi;
            }
        }
    }
}