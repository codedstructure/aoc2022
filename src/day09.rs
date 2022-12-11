use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

fn adjacent(head: (i32, i32), tail: (i32, i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn follow(head: (i32, i32), tail: (i32, i32), direction: &str) -> ((i32, i32), (i32, i32)) {
    let mut head = head;
    let mut tail = tail;
    match direction {
        "U" => {
            head = (head.0 + 1, head.1);
        }
        "D" => {
            head = (head.0 - 1, head.1);
        }
        "L" => {
            head = (head.0, head.1 - 1);
        }
        "R" => {
            head = (head.0, head.1 + 1);
        }
        _ => panic!("Unknown direction"),
    };
    tail = catch_up(head, tail);
    (head, tail)
}

fn catch_up(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut tail = tail;
    if adjacent(head, tail) {
        return tail;
    }
    if head.0 == tail.0 {
        tail.1 += (head.1 - tail.1).signum();
    } else if head.1 == tail.1 {
        tail.0 += (head.0 - tail.0).signum();
    } else {
        // differ in both dimensions
        tail.0 += (head.0 - tail.0).signum();
        tail.1 += (head.1 - tail.1).signum();
    }
    tail
}

pub fn step1() {
    // row, column; 'Up' is +ve.
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(tail);
    for rule in read_list("inputs/day09.txt") {
        let mut words = rule.split_whitespace();
        let direction = words.next().unwrap();
        let count = words.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..count {
            (head, tail) = follow(head, tail, direction);
            visited.insert(tail);
        }
    }

    println!("Visited cells: {}", visited.len());
}

pub fn step2() {
    // row, column; 'Up' is +ve.
    const ROPE_LEN: usize = 9;
    let mut head = (0, 0);
    let mut rope: Vec<(i32, i32)> = vec![]; // the 9 knots following the head.
    for _ in 0..ROPE_LEN {
        rope.push((0, 0));
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(rope[ROPE_LEN - 1]);
    for rule in read_list("inputs/day09.txt") {
        let mut words = rule.split_whitespace();
        let direction = words.next().unwrap();
        let count = words.next().unwrap().parse::<i32>().unwrap();
        println!("{} {}", direction, count);
        for _ in 0..count {
            (head, rope[0]) = follow(head, rope[0], direction);
            println!("Head {:?}", head);
            for knot in 1..ROPE_LEN {
                println!(
                    "knot {}, rope[{}] = {:?}, rope[{}] = {:?}",
                    knot,
                    knot,
                    rope[knot],
                    knot - 1,
                    rope[knot - 1]
                );
                if adjacent(rope[knot], rope[knot - 1]) {
                    break;
                }
                rope[knot] = catch_up(rope[knot - 1], rope[knot]);
            }
            println!("{:?}", rope);
            visited.insert(rope[ROPE_LEN - 1]);
        }

        for x in -10..10 {
            for y in -10..20 {
                if (9 - x, y) == head || rope.contains(&(9 - x, y)) {
                    print!("*");
                } else if (9 - x, y) == (0, 0) {
                    print!("s");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    println!("Full rope tail visited cells: {}", visited.len());
}
