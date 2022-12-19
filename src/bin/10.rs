use aoc::get_input;

fn main() {
    let lines = get_input();
    let mut reg = Vec::<i32>::new();
    let mut sum = 1i32;
    reg.push(999);
    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        reg.push(sum);
        if words[0] == "addx" {
            reg.push(sum);
            sum += words[1].parse::<i32>().unwrap();
        }
    }
    let mut total_signal_strength = 0i32;
    let mut index = 20;
    while index < reg.len() {
        total_signal_strength += (index as i32) * reg[index];
        index += 40;
    }
    println!("{}", total_signal_strength);
    for row in 0..6 {
        let mut line = Vec::<char>::new();
        for col in 0..40 {

            if (reg[row*40+col+1] - col as i32).abs() <= 1 {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        let s: String = line.iter().collect();
        println!("{}", s);
    }
}