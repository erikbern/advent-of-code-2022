use aoc::get_input;
use sscanf::sscanf;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Position {
    taken: u64,
    node: usize,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Score {
    total_flow: i32,
    flow: i32,
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

    // Initialize data
    let start: usize = str2int[&"AA".to_string()];
    let mut best = HashMap::<(u64, usize), i32>::new();
    let mut flows_h = HashMap::<u64, i32>::new();
    best.insert((0, start), 0);
    flows_h.insert(0u64, 0);

    // Run algo
    for t in 0..30 {
        let mut best_new = HashMap::<(u64, usize), i32>::new();
        let mut insert_maybe = |taken, node, total_flow| {
            let cur_total_flow: i32 = *best_new.get(&(taken, node)).unwrap_or(&0);
            if total_flow >= cur_total_flow {
                best_new.insert((taken, node), total_flow);
            }
        };
        for ((taken, node_a), total_flow) in &best {
            let flow = flows_h[taken];
            if taken & (1 << node_a) == 0 && flows[*node_a] > 0 {
                let taken2 = taken | (1 << node_a);
                flows_h.insert(taken2, flow + flows[*node_a]);
                insert_maybe(taken2, *node_a, total_flow + flow);
                
            }
            for node_b in &graph[*node_a] {
                insert_maybe(*taken, *node_b, *total_flow + flow);
            }
        }
        best = best_new;
        let max = best.values().max().unwrap();
        println!("step {}: total number of states: {} best: {}", t, best.len(), max);
    }
}