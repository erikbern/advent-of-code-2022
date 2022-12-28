use aoc::get_input;
use sscanf::sscanf;
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
struct Monkey {
    is_expr: bool,
    value: i64,
    term1: String,
    op: char,
    term2: String,
}

fn parse_monkey(line: &String) -> (String, Monkey) {
    let parsed = sscanf!(line, "{String}: {i64}");
    if parsed.is_ok() {
        let (id, value) = parsed.unwrap();
        (id, Monkey { is_expr: false, value, term1: "".to_string(), op: 'x', term2: "".to_string() } )
    } else {
        let parsed = sscanf!(line, "{String}: {String} {char} {String}");
        let (id, term1, op, term2) = parsed.unwrap();
        (id, Monkey { is_expr: true, value: 0, term1, op, term2 } )
    }
}

fn evaluate(monkeys: &HashMap::<String, Monkey>, id: &String, humn: Option::<i64>) -> f64 {
    let monkey: &Monkey = &monkeys[id];
    if humn.is_some() && id == "humn" {
        humn.unwrap() as f64
    } else if !monkey.is_expr {
        monkey.value as f64
    } else {
        let term1 = evaluate(&monkeys, &monkey.term1, humn) as f64;
        let term2 = evaluate(&monkeys, &monkey.term2, humn) as f64;
        if humn.is_some() && id == "root" {
            term1 - term2
        } else {
            match monkey.op {
                '+' => term1 + term2,
                '-' => term1 - term2,
                '*' => term1 * term2,
                '/' => term1 / term2,
                _ => panic!(),
            }
        }
    }
}

fn main() {
    let mut monkeys = HashMap::<String, Monkey>::new();
    for line in get_input() {
        let (id, monkey) = parse_monkey(&line);
        monkeys.insert(id, monkey);
    }
    let root = "root".to_string();

    // Part 1
    let ret = evaluate(&monkeys, &root, None);
    println!("{}", ret);

    // Part 2
    let mut humn = 0i64;
    let mut ret = evaluate(&monkeys, &root, Some(humn));
    let mut rng = rand::thread_rng();
    loop {
        let sign = if rng.gen() { 1i64 } else { -1i64 };
        let exp: u32 = rng.gen_range(0..40);
        let new_humn = humn + sign * 2i64.pow(exp);
        let new_ret = evaluate(&monkeys, &root, Some(new_humn));
        if new_ret.abs() < ret.abs() {
            println!("{} -> {}", new_humn, new_ret);
            humn = new_humn;
            ret = new_ret;
            if ret == 0f64 {
                break;
            }
        }
    }
}