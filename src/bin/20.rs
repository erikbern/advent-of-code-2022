use aoc::get_input;

fn main() {
    let mut items = Vec::<i32>::new();
    let mut order = Vec::<usize>::new();
    for (i, line) in get_input().iter().enumerate() {
        items.push(line.parse::<i32>().unwrap());
        order.push(i);
    }
    let n = items.len() as i32;
    println!("n = {}", n);
    for i in 0..items.len() {
        let p: usize = order.iter().position(|v| *v == i).unwrap();
        let mut p: i32 = p.try_into().unwrap();
        let mut new_p = p + items[i];
        if new_p <= 0 {
            new_p += n - 1;
        } else if new_p >= n {
            new_p -= n - 1;
        }
        let steps = (new_p - p).abs();
        let d = if new_p > p { 1 } else { -1 };
        for j in 0..steps {
            let q = (p + d + n) % n;
            order.swap(p as usize, q as usize);
            p = q;
        }
        let z: Vec::<i32> = order.iter().map(|i| items[*i]).collect();
    }
    let i0: usize = items.iter().position(|v| *v == 0).unwrap();
    let p: usize = order.iter().position(|v| *v == i0).unwrap();
    let n = items.len();
    println!("{}", items[order[(p + 1000) % n]] + items[order[(p + 2000) % n]] + items[order[(p + 3000) % n]]);
}