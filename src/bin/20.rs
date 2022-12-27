use aoc::get_input;

fn mix(items: &Vec<i64>, order: &mut Vec<usize>) {
    let n = items.len() as i64;
    for i in 0..items.len() {
        let p: usize = order.iter().position(|v| *v == i).unwrap();
        let mut p: i64 = p.try_into().unwrap();
        let new_p = (p + items[i]).rem_euclid(n - 1);
        let steps = (new_p - p).abs();
        let d = if new_p > p { 1 } else { -1 };
        for _j in 0..steps {
            let q = (p + d + n) % n;
            order.swap(p as usize, q as usize);
            p = q;
        }
    }
}

fn get_new_items(items: &Vec<i64>, order: &Vec<usize>) -> Vec::<i64> {
    order.iter().map(|i| items[*i]).collect()
}

fn checksum(items: &Vec<i64>, order: &Vec<usize>) -> i64 {
    let new_items = get_new_items(items, order); 
    let p: usize = new_items.iter().position(|v| *v == 0).unwrap();
    let n = items.len();
    new_items[(p + 1000) % n] + new_items[(p + 2000) % n] + new_items[(p + 3000) % n]
}

fn main() {
    let items: Vec<i64> = get_input().iter().map(|line| line.parse::<i64>().unwrap()).collect();
    let n = items.len();

    // Part 1
    let mut order = (0..n).collect();
    mix(&items, &mut order);
    println!("{}", checksum(&items, &order));

    // Part 2
    let items = items.iter().map(|item| item * 811589153).collect();
    let mut order = (0..n).collect();
    for _i in 0..10 {
        mix(&items, &mut order);
        if n < 20 {
            println!("{:?}", get_new_items(&items, &order));
        }
    }
    println!("{}", checksum(&items, &order));
}