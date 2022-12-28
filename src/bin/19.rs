use aoc::get_input;
use std::collections::HashSet;
use regex::Regex;
use std::cmp::min;

type M = [i32; 4];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct S {
    res: M,
    rob: M,
}

#[derive(Debug)]
struct Recipe {
    material: usize,
    cost: M,
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    recipes: Vec<Recipe>,
}

fn add(a: &M, b: &M) -> M {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

fn sub(a: &M, b: &M) -> M {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

fn is_ge(a: &M, b: &M) -> bool {
    a[0] >= b[0] && a[1] >= b[1] && a[2] >= b[2] && a[3] >= b[3]
}

fn incr_index(a: &M, index: usize) -> M {
    let mut b = a.clone();
    b[index] += 1;
    b
}

fn search(recipes: &Vec<Recipe>, n_steps: i32) -> i32 {
    let res1 = [0, 0, 0, 0];
    let rob1 = [1, 0, 0, 0];
    let s = S { res: res1, rob: rob1 };
    let mut m = HashSet::<S>::new();
    m.insert(s);
    for step in 0..n_steps {
        // println!("step {}: {} states", step, m.len());
        let mut m_new = HashSet::<S>::new();
        let mut best_lower_bound = 0;
        for s in &m {
            let n_steps_left = n_steps - step - 1;
            let lower_bound = s.res[3] + s.rob[3] * n_steps_left;
            let upper_bound = s.res[3] + s.rob[3] * n_steps_left + n_steps_left * (n_steps_left - 1) / 2;
            if lower_bound > best_lower_bound {
                best_lower_bound = lower_bound;
            } else if upper_bound < best_lower_bound {
                continue;
            }
            let res_n = add(&s.res, &s.rob);
            for r in recipes {
                if is_ge(&s.res, &r.cost) {
                    m_new.insert(S {
                        res: sub(&res_n, &r.cost),
                        rob: incr_index(&s.rob, r.material),
                    });
                }
            }
            m_new.insert(S {res: res_n, rob: s.rob });
        }
        m = m_new;
    }   
    m.iter().map(|s| s.res[3]).max().unwrap()
}

fn main() {
    let regex = Regex::new(r"\d+").unwrap();
    let blueprints: Vec<Blueprint> = get_input().iter().map(|line| {
        let ints: Vec<i32> = regex.find_iter(&line).map(|item| item.as_str().parse::<i32>().unwrap()).collect();
        let id: i32 = ints[0];
        let recipes = vec![
            Recipe {material: 0, cost: [ints[1], 0, 0, 0]},
            Recipe {material: 1, cost: [ints[2], 0, 0, 0]}, 
            Recipe {material: 2, cost: [ints[3], ints[4], 0, 0]},
            Recipe {material: 3, cost: [ints[5], 0, ints[6], 0]},
        ];
        Blueprint {id, recipes}
    }).collect();

    // Part 1
    let mut total_quality_level = 0i32;
    for blueprint in &blueprints {
        let n_geos = search(&blueprint.recipes, 24);
        println!("{}: {}", blueprint.id, n_geos);
        total_quality_level += blueprint.id * n_geos;
    }
    println!("-> {}", total_quality_level);

    // Part 2
    let mut product = 1i32;
    let k = min(blueprints.len(), 3);
    for blueprint in &blueprints[..k] {
        let n_geos = search(&blueprint.recipes, 32);
        println!("{}: {}", blueprint.id, n_geos);
        product *= n_geos;
    }
    println!("-> {}", product);
}