use aoc::get_input;
use std::collections::HashMap;

fn main() {
    let lines = get_input();
    let mut stack = Vec::<String>::new();
    let mut file_sizes = HashMap::<Vec::<String>, i32>::new();
    for line in lines {
        let words: Vec<String> = line.split(" ").map(str::to_string).collect();
        if words[0] == "$" && words[1] == "cd" {
            if words[2] == "/" {
                stack.clear();
            } else if words[2] == ".." {
                stack.pop();
            } else {
                stack.push(words[2].clone());
            }
        } else if words[0] == "$" && words[1] == "ls" {
        } else if words[0] == "dir" {
        } else {
            let filename = words[1].clone();
            let mut stack_copy = stack.clone();
            stack_copy.push(filename);
            let size: i32 = words[0].parse::<i32>().unwrap();
            file_sizes.insert(stack_copy, size);
        }
    }
    let mut dir_sizes = HashMap::<Vec::<String>, i32>::new();
    for (file, size) in &file_sizes {
        let mut dir = file.clone();
        while dir.len() > 0 {
            dir.pop();
            let total = dir_sizes.entry(dir.clone()).or_default();
            *total += size;
        }
    }
    let mut total_small = 0i32;
    let root = Vec::<String>::new();
    let root_size = dir_sizes.get(&root).unwrap();
    let needed = root_size - 40000000;
    let mut smallest = 0i32;
    for (_dir, size) in &dir_sizes {
        if size <= &100000 {
            total_small += size;
        }
        if size >= &needed {
            if smallest == 0 || size < &smallest {
                smallest = *size;
            }
        }
    }
    println!("{} {}", total_small, smallest);
}
