use aoc::get_input;
use sscanf::sscanf;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Position {
    taken: u64,
    node_1: usize,
    node_2: usize,
}

fn main() {
    // Read input
    let input: Vec<(String, i32, Vec<String>)> = get_input().iter().map(|line| {
        let parsed = sscanf!(line, "Valve {str} has flow rate={i32}; {str:/tunnels? leads? to valves?/} {str}");
        let (node_a, flow, _, tunnels) = parsed.unwrap();
        let nodes_b: Vec<String> = tunnels.split(", ").map(|s| s.to_string()).collect();
        (node_a.to_string(), flow, nodes_b)
    }).collect();

    // Map strings to ints
    let str2int: HashMap<String, usize> = input.iter().map(|(node, _, _)| node).enumerate().map(|(i, node)| (node.clone(), i)).collect();

    // Remap input
    let mut flows = vec![0i32; str2int.len()];
    let mut graph = vec![Vec::<usize>::new(); str2int.len()];
    for (node_a, flow, nodes_b) in input {
        let node_a: usize = str2int[&node_a];
        flows[node_a] = flow;
        for node_b in nodes_b {
            let node_b: usize = str2int[&node_b];
            graph[node_a].push(node_b);
        }
    }
    let flows_sum: i32 = flows.iter().sum();

    // Generate moves from a position
    let get_moves = |taken: u64, node: usize| {
        let mut moves = Vec::<(u64, usize)>::new();
        if taken & (1 << node) == 0 && flows[node] > 0 {
            let new_taken = taken | (1 << node);
            moves.push((new_taken, node));
        }
        for node_b in &graph[node] {
            moves.push((taken, *node_b));
        }
        moves
    };

    for mode in 0..2 {
        // Initialize data
        let start: usize = str2int[&"AA".to_string()];
        let mut best = HashMap::<Position, i32>::new();
        best.insert(Position { taken: 0u64, node_1: start, node_2: start }, 0);

        // Run algo
        let n_steps = if mode == 0 { 30 } else { 26 };
        for t in 0..n_steps {
            let mut best_new = HashMap::<Position, i32>::new();
            let mut insert_or_update = |p, v| {
                best_new.entry(p).and_modify(|ev| { *ev = max(v, *ev); } ).or_insert(v);
            };
            let mut best_lower_bound = 0;
            for (pos, total_flow) in &best {
                let flow: i32 = flows.iter().enumerate().map(|(i, f)| {
                    if pos.taken & (1 << i) != 0 { *f } else { 0i32 }
                }).sum();

                let lower_bound = total_flow + (n_steps - 1 - t) * flow;
                best_lower_bound = max(best_lower_bound, lower_bound);

                let upper_bound = total_flow + (n_steps - 1 - t) * flows_sum;
                if upper_bound < best_lower_bound {
                    continue;
                }

                if mode == 0 {
                    for (taken, node) in get_moves(pos.taken, pos.node_1) {
                        let new_pos = Position { taken: taken, node_1: node, node_2: 0 };
                        insert_or_update(new_pos, total_flow + flow);
                    }
                }
                if mode == 1 {
                    for (taken_1, node_1) in get_moves(pos.taken, pos.node_1) {
                        for (taken_2, node_2) in get_moves(taken_1, pos.node_2) {
                            let new_pos = Position { taken: taken_1 | taken_2, node_1: node_1, node_2: node_2 };
                            insert_or_update(new_pos, total_flow + flow);
                        }
                    }
                }
            }
            best = best_new;
            let max = best.values().max().unwrap();
            println!("step {}: total number of states: {} best: {}", t, best.len(), max);
        }
    }
}