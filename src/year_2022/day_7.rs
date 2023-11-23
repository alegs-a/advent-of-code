#![allow(dead_code)]
use std::{collections::HashMap, fs};

pub fn day_seven() {
    println!("DAY 7 ==============");

    let raw_input = fs::read_to_string("input/7.txt").expect("error reading input file");

    println!("PART 1: {}", part_one(raw_input.clone()));
    println!("PART 2: {}", part_two(raw_input.clone()));
}

pub fn part_1(raw_input: String) -> String {
    let scores = generate_scores(raw_input);

    let mut total = 0;
    for (_path, size) in scores {
        if size <= 100000 {
            total += size;
        }
    }
    total.to_string()
}

pub fn part_2(raw_input: String) -> String {
    let scores = generate_scores(raw_input);

    let used = scores.get("/").unwrap();
    let free = 70_000_000 - used;
    let required = 30_000_000 - free;
    let mut smallest_possible = free;

    for (_path, size) in scores {
        if size > required && size < smallest_possible {
            smallest_possible = size;
        }
    }
    smallest_possible.to_string()
}
fn part_one(raw_input: String) -> i32 {
    let scores = generate_scores(raw_input);

    let mut total = 0;
    for (_path, size) in scores {
        if size <= 100000 {
            total += size;
        }
    }
    total
}

fn part_two(raw_input: String) -> i32 {
    let scores = generate_scores(raw_input);

    let used = scores.get("/").unwrap();
    let free = 70_000_000 - used;
    let required = 30_000_000 - free;
    let mut smallest_possible = free;

    for (_path, size) in scores {
        if size > required && size < smallest_possible {
            smallest_possible = size;
        }
    }
    smallest_possible
}

fn generate_scores(raw_input: String) -> HashMap<String, i32> {
    let lines = raw_input.lines();

    let mut scores: HashMap<String, i32> = HashMap::new();
    let mut paths: Vec<String> = vec!["/".to_string()];

    for line in lines {
        let words = line.split(' ').collect::<Vec<&str>>();
        match words[0] {
            "$" => {
                if words[1] == "cd" {
                    // need to catch ..
                    if words[2] == ".." {
                        paths.pop();
                    } else {
                        let new_path = format!("{}{}", paths.last().unwrap(), words[2]);
                        paths.push(new_path.clone());
                    }
                }
            }
            "dir" => {}
            _ => {
                let size = words[0].parse::<i32>().unwrap();
                for path in &paths {
                    let score = scores.entry(path.to_string()).or_insert(0);
                    *score += size
                }
            }
        }
    }
    scores
}

// :(
// /// A graph.
// #[derive(Debug)]
// struct Tree<'a> {
//     /// Hashmap of vertices, key is vertex ID and value is vertex size
//     vertices: HashMap<&'a str, i32>,
//     /// Hashmap of edges, key is origin vertex and value is `Vec<>` of destination vertices
//     children: HashMap<&'a str, Vec<&'a str>>
// }
//
// impl<'a> Tree<'a> {
//     /// Returns an empty graph.
//     fn new() -> Tree<'a> {
//         Tree {
//             vertices: HashMap::new(),
//             children: HashMap::new(),
//         }
//     }
//
//     /// Adds a new node to an existing graph
//     fn add_child(&mut self, new: &'a str, from: &'a str, size: i32) {
//         self.vertices.insert(new, size).unwrap(); // input should not have any duplicate nodes
//         self.children.entry(from)
//             .and_modify(|x| x.push(new))
//             .or_insert(vec![new]);
//     }
// }
