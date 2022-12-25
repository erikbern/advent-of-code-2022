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
    let row_y = env::args().nth(2).unwrap().parse::<i32>().unwrap();
    let mut sensors = Vec::<Sensor>::new();
    let mut beacons_in_row = HashSet::<i32>::new();
    for line in lines {
        let parsed = sscanf!(line, "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}");
        let (sx, sy, bx, by) = parsed.unwrap();
        sensors.push(Sensor {x: sx, y: sy, r: (bx - sx).abs() + (by - sy).abs()});
        if by == row_y {
            beacons_in_row.insert(by);
        }
    }
    println!("{:?}", sensors);
    let mut taken = HashSet::<i32>::new();
    for sensor in sensors {
        let dy = (sensor.y - row_y).abs();
        if sensor.r >= dy {
            let x_lo = sensor.x - (sensor.r - dy);
            let x_hi = sensor.x + (sensor.r - dy);
            println!("{},{}: {} - {}", sensor.x, sensor.y, x_lo, x_hi);
            for x in x_lo..=x_hi {
                taken.insert(x);
            }
        }
    }
    println!("{}", taken.difference(&beacons_in_row).count());
}