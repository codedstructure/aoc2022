use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

pub fn step1() {
    let mut cycle = 0;
    let mut reg_x = 1;
    let mut signal_strength = 0;
    for instruction in read_list("inputs/day10.txt") {
        if instruction.starts_with("noop") {
            cycle += 1;
            if cycle % 40 == 20 {
                signal_strength += reg_x * cycle;
                println!("cycle {}, {} strength {}", cycle, reg_x, signal_strength);
            }
        } else {
            cycle += 1;
            if cycle % 40 == 20 {
                signal_strength += reg_x * cycle;
                println!("cycle {}, {} strength {}", cycle, reg_x, signal_strength);
            }
            cycle += 1;
            if cycle % 40 == 20 {
                signal_strength += reg_x * cycle;
                println!("cycle {}, {} strength {}", cycle, reg_x, signal_strength);
            }
            reg_x += instruction
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }
    println!("Signal strength: {}", signal_strength);
}

pub fn step2() {
    let mut display: Vec<char> = vec![];
    let mut cycle = -1;
    let mut reg_x = 1;
    for instruction in read_list("inputs/day10.txt") {
        if instruction.starts_with("noop") {
            cycle += 1;
            if cycle % 40 == 0 {
                for c in &display {
                    print!("{}", c);
                }
                display.clear();
                println!();
            }
            display.push(if ((reg_x - cycle % 40) as i32).abs() <= 1 {
                '#'
            } else {
                ' '
            });
        } else {
            cycle += 1;
            if cycle % 40 == 0 {
                for c in &display {
                    print!("{}", c);
                }
                display.clear();
                println!();
            }
            display.push(if ((reg_x - cycle % 40) as i32).abs() <= 1 {
                '#'
            } else {
                ' '
            });
            cycle += 1;
            if cycle % 40 == 0 {
                for c in &display {
                    print!("{}", c);
                }
                display.clear();
                println!();
            }
            display.push(if ((reg_x - cycle % 40) as i32).abs() <= 1 {
                '#'
            } else {
                ' '
            });
            reg_x += instruction
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }
    for c in &display {
        print!("{}", c);
    }
    display.clear();
    println!();
}
