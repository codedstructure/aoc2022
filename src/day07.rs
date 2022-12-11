use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

pub fn step1() {
    let mut dir_stack: Vec<String> = vec![];
    let mut sizes: HashMap<String, usize> = HashMap::new();
    for line in read_list("inputs/day07.txt") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.len() {
            2 => match parts[0] {
                "dir" => (),
                "$" => match parts[1] {
                    "ls" => (),
                    _ => panic!("Unknown command {}", parts[1]),
                },
                _ => {
                    let size: usize = parts[0].parse().unwrap();
                    let _fname = parts[1];

                    let mut pwd_stack = dir_stack.clone();
                    while !pwd_stack.is_empty() {
                        let path = "/".to_string() + &pwd_stack.join("/");
                        *sizes.entry(path).or_insert(0) += size;
                        pwd_stack.pop();
                    }
                    *sizes.entry("/".to_string()).or_insert(0) += size;
                }
            },
            3 => match parts[0] {
                "$" => match parts[1] {
                    "cd" => match parts[2] {
                        "/" => dir_stack.clear(),
                        ".." => {
                            dir_stack.pop();
                        }
                        _ => dir_stack.push(parts[2].to_string()),
                    },
                    _ => panic!("Unknown command {}", parts[1]),
                },
                _ => panic!("Unknown entry {:?}", parts),
            },
            _ => panic!("Unknown entry: {:?}", parts),
        }
    }

    let mut total = 0;
    for value in sizes.values() {
        if *value <= 100000 {
            total += value;
        }
    }
    println!("Total size of directories < 100000 is {}", total);
}

pub fn step2() {
    let mut dir_stack: Vec<String> = vec![];
    let mut sizes: HashMap<String, usize> = HashMap::new();
    for line in read_list("inputs/day07.txt") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.len() {
            2 => match parts[0] {
                "dir" => (),
                "$" => match parts[1] {
                    "ls" => (),
                    _ => panic!("Unknown command {}", parts[1]),
                },
                _ => {
                    let size: usize = parts[0].parse().unwrap();
                    let _fname = parts[1];

                    let mut pwd_stack = dir_stack.clone();
                    while !pwd_stack.is_empty() {
                        let path = "/".to_string() + &pwd_stack.join("/");
                        *sizes.entry(path).or_insert(0) += size;
                        pwd_stack.pop();
                    }
                    *sizes.entry("/".to_string()).or_insert(0) += size;
                }
            },
            3 => match parts[0] {
                "$" => match parts[1] {
                    "cd" => match parts[2] {
                        "/" => dir_stack.clear(),
                        ".." => {
                            dir_stack.pop();
                        }
                        _ => dir_stack.push(parts[2].to_string()),
                    },
                    _ => panic!("Unknown command {}", parts[1]),
                },
                _ => panic!("Unknown entry {:?}", parts),
            },
            _ => panic!("Unknown entry: {:?}", parts),
        }
    }

    let remaining = 70000000 - sizes["/"];
    let min_dir_sizes_to_delete = 30000000 - remaining;
    println!(
        "remaining space: {}. Target to delete: {}",
        remaining, min_dir_sizes_to_delete
    );

    let mut smallest = usize::MAX;
    for value in sizes.values() {
        if *value < smallest && *value > min_dir_sizes_to_delete {
            smallest = *value;
        }
    }
    println!("Size of smallest big directory: {}", smallest);
}
