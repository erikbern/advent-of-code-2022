use aoc::get_input;
use std::collections::VecDeque;
use num::integer::lcm;

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    term_a: String,
    op: char,
    term_b: String,
    modulo: u64,
    monkey_true: usize,
    monkey_false: usize,
    inspected: u64,
}

fn get_term(old: u64, term: &String) -> u64 {
    if term == "old" {
        old
    } else {
        term.parse::<u64>().unwrap()
    }
}

fn simulate(_monkeys: &Vec<Monkey>, n_rounds: u64, multiple: u64, shrink: u64) -> u64 {
    let mut monkeys = _monkeys.clone();
    let n_monkeys = monkeys.len();
    for _round in 0..n_rounds {
        for i in 0..n_monkeys {
            while let Some(item) = monkeys[i].items.pop_front() {
                let term_a = get_term(item, &monkeys[i].term_a);
                let term_b = get_term(item, &monkeys[i].term_b);
                let mut result = match monkeys[i].op {
                    '*' => term_a * term_b,
                    '+' => term_a + term_b,
                    _ => 0,
                };
                if shrink > 0 {
                    result /= shrink;
                }
                if multiple > 0 {
                    result %= multiple;
                }
                let monkey_to = match result % monkeys[i].modulo {
                    0 => monkeys[i].monkey_true,
                    _ => monkeys[i].monkey_false,
                };
                monkeys[monkey_to].items.push_back(result);
                monkeys[i].inspected += 1;
            }
        }
    }

    let mut inspected: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect();
    inspected.sort();
    inspected[n_monkeys-2] * inspected[n_monkeys-1]
}

fn main() {
    let lines = get_input();
    let n_monkeys = (lines.len() + 1) / 7;
    let mut monkeys = Vec::<Monkey>::new();

    let mut multiple = 1u64;
    for i in 0..n_monkeys {
        // Parse items
        let (_, items) = lines[i*7+1].split_once(": ").unwrap();
        let items: VecDeque<u64> = items.split(", ").map(|w| w.parse::<u64>().unwrap()).collect();

        // Parse operation
        let (_, operation) = lines[i*7+2].split_once(": ").unwrap();
        let parsed = sscanf::sscanf!(operation, "new = {} {} {}", String, char, String);
        let (term_a, op, term_b) = parsed.unwrap();

        // Parse test
        let parsed = sscanf::sscanf!(lines[i*7+3], "  Test: divisible by {}", u64);
        let modulo = parsed.unwrap();

        // Parse actions
        let parsed = sscanf::scanf!(lines[i*7+4], "    If true: throw to monkey {}", usize);
        let monkey_true = parsed.unwrap();
        let parsed = sscanf::scanf!(lines[i*7+5], "    If false: throw to monkey {}", usize);
        let monkey_false = parsed.unwrap();

        let inspected = 0u64;
        multiple = lcm(modulo, multiple);
        monkeys.push(Monkey{items, term_a, op, term_b, modulo, monkey_true, monkey_false, inspected});
    }

    println!("{}", multiple);
    println!("{}", simulate(&monkeys, 20, 0, 3));
    println!("{}", simulate(&monkeys, 10000, multiple, 0));
}