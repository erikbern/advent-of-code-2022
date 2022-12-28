use aoc::get_input;
use std::collections::HashSet;
use regex::Regex;
use std::cmp::min;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct R(i32, i32, i32, i32); // ore, clay, obsidian, geode

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct S {
    res: R,
    rob: R,
}

struct Input {
    ore_ore: i32,
    cla_ore: i32,
    obs_ore: i32,
    obs_cla: i32,
    geo_ore: i32,
    geo_obs: i32,
}

fn search(i: &Input, n_steps: i32) -> i32 {
    let res1 = R(0, 0, 0, 0);
    let rob1 = R(1, 0, 0, 0);
    let s = S { res: res1, rob: rob1 };
    let mut m = HashSet::<S>::new();
    m.insert(s);
    for step in 0..n_steps {
        // println!("step {}: {} states", step, m.len());
        let mut m_new = HashSet::<S>::new();
        let mut best_lower_bound = 0;
        for s in &m {
            let n_steps_left = n_steps - step - 1;
            let lower_bound = s.res.3 + s.rob.3 * n_steps_left;
            let upper_bound = s.res.3 + s.rob.3 * n_steps_left + n_steps_left * (n_steps_left - 1) / 2;
            if lower_bound > best_lower_bound {
                best_lower_bound = lower_bound;
            } else if upper_bound < best_lower_bound {
                continue;
            }
            let res_n = R(s.res.0 + s.rob.0, s.res.1 + s.rob.1, s.res.2 + s.rob.2, s.res.3 + s.rob.3);
            if s.res.0 >= i.ore_ore {
                m_new.insert(S {
                    res: R(res_n.0 - i.ore_ore, res_n.1, res_n.2, res_n.3),
                    rob: R(s.rob.0 + 1, s.rob.1, s.rob.2, s.rob.3),
                });
            }
            if s.res.0 >= i.cla_ore {
                m_new.insert(S {
                    res: R(res_n.0 - i.cla_ore, res_n.1, res_n.2, res_n.3),
                    rob: R(s.rob.0, s.rob.1 + 1, s.rob.2, s.rob.3),
                });
            }
            if s.res.0 >= i.obs_ore && s.res.1 >= i.obs_cla {
                m_new.insert(S {
                    res: R(res_n.0 - i.obs_ore, res_n.1 - i.obs_cla, res_n.2, res_n.3),
                    rob: R(s.rob.0, s.rob.1, s.rob.2 + 1, s.rob.3),
                });
            }
            if s.res.0 >= i.geo_ore && s.res.2 >= i.geo_obs {
                m_new.insert(S {
                    res: R(res_n.0 - i.geo_ore, res_n.1, res_n.2 - i.geo_obs, res_n.3),
                    rob: R(s.rob.0, s.rob.1, s.rob.2, s.rob.3 + 1),
                });
            }
            m_new.insert(S {res: res_n, rob: s.rob });
        }
        m = m_new;
    }
    m.iter().map(|s| s.res.3).max().unwrap()
}

fn main() {
    let regex = Regex::new(r"\d+").unwrap();
    let mut inputs = Vec::<(Input, i32)>::new();

    for line in get_input() {
        let ints: Vec<i32> = regex.find_iter(&line).map(|item| item.as_str().parse::<i32>().unwrap()).collect();
        let b_id: i32 = ints[0];
        let input = Input {ore_ore: ints[1], cla_ore: ints[2], obs_ore: ints[3], obs_cla: ints[4], geo_ore: ints[5], geo_obs: ints[6] };
        inputs.push((input, b_id));
    }

    let mut total_quality_level = 0i32;
    for (input, b_id) in &inputs {
        let n_geos = search(&input, 24);
        println!("{}: {}", b_id, n_geos);
        total_quality_level += b_id * n_geos;
    }
    println!("-> {}", total_quality_level);

    let mut product = 1i32;
    let k = min(inputs.len(), 3);
    for (input, b_id) in &inputs[..k] {
        let n_geos = search(&input, 32);
        println!("{}: {}", b_id, n_geos);
        product *= n_geos;
    }
    println!("-> {}", product);
}