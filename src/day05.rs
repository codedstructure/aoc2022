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
    let mut rules: Vec<(i32, i32, i32)> = vec![];
    let mut stack: HashMap<i32, Vec<char>> = HashMap::new();
    for line in read_list("inputs/day05.txt") {
        if line.starts_with('[') {
            for entry in 0..9 {
                if let Some(c) = line.chars().nth(1 + entry * 4) {
                    if c == ' ' {
                        continue;
                    };
                    let v: &mut Vec<char> = stack.entry(1 + entry as i32).or_default();
                    v.insert(0, c);
                }
            }
        } else if line.starts_with("move") {
            let mut line_iter = line.split(' ');
            // extract elements 1, 3, 5 - note nth() consumes, so we're just skipping one
            // each time to get these.
            let count = line_iter.nth(1).unwrap().parse::<i32>().unwrap();
            let source = line_iter.nth(1).unwrap().parse::<i32>().unwrap();
            let dest = line_iter.nth(1).unwrap().parse::<i32>().unwrap();
            rules.push((count, source, dest));
        }
    }
    for (count, source, dest) in rules {
        for _ in 0..count {
            let source_stack: &mut Vec<char> = stack.get_mut(&(source as i32)).unwrap();
            let element = source_stack.pop().unwrap();
            let dest_stack: &mut Vec<char> = stack.get_mut(&(dest as i32)).unwrap();
            dest_stack.push(element);
        }
    }
    for entry in 1..=9 {
        print!("{}", stack.get(&entry).unwrap().last().unwrap());
    }
    println!();
}

pub fn step2() {
    let mut rules: Vec<(i32, i32, i32)> = vec![];
    let mut stack: HashMap<i32, Vec<char>> = HashMap::new();
    for line in read_list("inputs/day05.txt") {
        if line.starts_with('[') {
            for entry in 0..9 {
                if let Some(c) = line.chars().nth(1 + entry * 4) {
                    if c == ' ' {
                        continue;
                    };
                    let v: &mut Vec<char> = stack.entry(1 + entry as i32).or_default();
                    v.insert(0, c);
                }
            }
        } else if line.starts_with("move") {
            let mut line_iter = line.split(' ');
            // extract elements 1, 3, 5 - note nth() consumes, so we're just skipping one
            // each time to get these.
            let count = line_iter.nth(1).unwrap().parse::<i32>().unwrap();
            let source = line_iter.nth(1).unwrap().parse::<i32>().unwrap();
            let dest = line_iter.nth(1).unwrap().parse::<i32>().unwrap();
            rules.push((count, source, dest));
        }
    }
    for (count, source, dest) in rules {
        let mut elements: Vec<char> = vec![];
        let source_stack: &mut Vec<char> = stack.get_mut(&(source as i32)).unwrap();
        for _ in 0..count {
            let element = source_stack.pop().unwrap();
            elements.push(element);
        }
        let dest_stack: &mut Vec<char> = stack.get_mut(&(dest as i32)).unwrap();
        while !elements.is_empty() {
            dest_stack.push(elements.pop().unwrap());
        }
    }
    for entry in 1..=9 {
        print!("{}", stack.get(&entry).unwrap().last().unwrap());
    }
    println!();
}
