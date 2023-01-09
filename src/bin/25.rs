use aoc::get_input;

fn dig(ch: char) -> i64 {
    match ch {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }
}

fn snafu2dec(s: &String) -> i64 {
    let mut pow5 = 1i64;
    let mut val = 0i64;
    for ch in s.chars().rev() {
        val += dig(ch) * pow5;
        pow5 *= 5;
    }
    val
}

fn dec2snafu(mut val: i64) -> String {
    let mut out = Vec::<char>::new();
    let digs = ['0', '1', '2', '=', '-'];
    while val != 0 {
        let last_dig: i64 = val.rem_euclid(5);
        out.push(digs[last_dig as usize]);
        let last_dig = if last_dig >= 3 { last_dig - 5 } else { last_dig };
        val = (val - last_dig) / 5;
    }
    out.iter().rev().collect()
}

fn main() {
    let mut sum = 0i64;
    for line in get_input() {
        sum += snafu2dec(&line);
    }
    println!("{}", sum);
    println!("{}", dec2snafu(sum));
}