use aoc::get_input;
use std::slice;
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    is_list: bool,
    integer: i32,
    nodes: Vec<Node>,
    divider: i32,
}

fn integer(k: i32) -> Node {
    Node {is_list: false, integer: k, nodes: vec![], divider: 0}
}

fn parse(line: &[u8]) -> (Node, &[u8]) {
    // println!("parsing {:?}", line);
    let mut pt = line;
    if pt[0] == '[' as u8 {
        let mut nodes = Vec::<Node>::new();
        pt = &pt[1..]; // consume the [
        while pt[0] != ']' as u8 {
            let (node, pt_new) = parse(&pt);
            nodes.push(node);
            pt = pt_new;
            if pt[0] == ',' as u8 { pt = &pt[1..]};
        }
        (Node {is_list: true, integer: -1, nodes, divider: 0}, &pt[1..])
    } else {
        let mut k = 0i32;
        while pt[0] >= '0' as u8 && pt[0] <= '9' as u8 {
            k = 10 * k + (pt[0] - '0' as u8) as i32;
            pt = &pt[1..];
        }
        (integer(k), pt)
    }
}

fn cmp_int(a: i32, b: i32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn cmp_slice(a: &[Node], b: &[Node]) -> Ordering {
    if a.len() == 0 && b.len() == 0 {
        Ordering::Equal
    } else if a.len() == 0 && b.len() > 0 {
        Ordering::Less
    } else if a.len() > 0 && b.len() == 0 {
        Ordering::Greater
    } else {
        let head = cmp(&a[0], &b[0]);
        if head != Ordering::Equal {
            head
        } else {
            cmp_slice(&a[1..], &b[1..])
        }
    }
}

fn cmp(a: &Node, b: &Node) -> Ordering {
    //println!("comparing {:?} and {:?}", a, b);
    if !a.is_list && !b.is_list {
        cmp_int(a.integer, b.integer)
    } else if a.is_list && b.is_list {
        cmp_slice(&a.nodes[..], &b.nodes[..])
    } else if a.is_list && !b.is_list {
        cmp_slice(&a.nodes[..], slice::from_ref(b))
    } else if !a.is_list && b.is_list {
        cmp_slice(slice::from_ref(a), &b.nodes[..])
    } else {
        panic!("unreachable");
    }
}

fn wrap(node: Node, divider: i32) -> Node {
    Node { is_list: true, integer: -1, nodes: vec![node], divider: divider  }
}

fn main() {
    let lines = get_input();
    let n_pairs = (lines.len() + 1)/3;
    let mut sum = 0i32;
    let mut nodes = Vec::<Node>::new();
    for pair in 0..n_pairs {
        let (pair_a, _) = parse(&lines[3*pair].as_bytes());
        let (pair_b, _) = parse(&lines[3*pair + 1].as_bytes());
        if cmp(&pair_a, &pair_b) == Ordering::Less {
            sum += (pair + 1) as i32;
        }
        nodes.push(pair_a);
        nodes.push(pair_b);
    }
    println!("{:?}", sum);

    nodes.push(wrap(wrap(integer(2), 0), 1));
    nodes.push(wrap(wrap(integer(6), 0), 2));
    nodes.sort_by(cmp);
    let div1 = nodes.iter().position(|n| n.divider == 1).unwrap();
    let div2 = nodes.iter().position(|n| n.divider == 2).unwrap();
    println!("{}", (div1 + 1) * (div2 + 1));
}