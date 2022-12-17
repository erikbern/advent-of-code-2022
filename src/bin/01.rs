use aoc::get_input;

fn main() {
    let lines = get_input();
    let mut sum: i32 = 0;
    let mut elves = Vec::<i32>::new();
    for line in lines {
        if line == "" {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    elves.sort();
    println!("{}", elves.last().unwrap());
    println!("{}", elves[elves.len()-3..].iter().sum::<i32>());
}
