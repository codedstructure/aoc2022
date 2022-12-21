use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
enum Monkey {
    Fact(f64),
    Computation(String, String, String),
}

fn evaluate(root: String, monkeys: &HashMap<String, Monkey>, human: Option<f64>) -> f64 {
    if let Some(humn) = human {
        if root.as_str() == "humn" {
            return humn;
        }
    }
    match monkeys.get(&root).unwrap() {
        Monkey::Fact(i) => *i,
        Monkey::Computation(m1, op, m2) => {
            let m1 = evaluate(m1.clone(), monkeys, human);
            let m2 = evaluate(m2.clone(), monkeys, human);
            match op.as_str() {
                "+" => m1 + m2,
                "-" => m1 - m2,
                "*" => m1 * m2,
                "/" => m1 / m2,
                "=" => {
                    assert!(human.is_some());
                    m1 - m2
                }
                _ => panic!("Unknown operation"),
            }
        }
    }
}

fn solve_for_human(monkeys: &HashMap<String, Monkey>) -> i64 {
    // Arbitrary big 2^n number. Needs to be big to maximise O(logN)
    // portion vs O(n) portion, and 2^n so can repeatedly half and
    // end up at 1.
    let mut mult = (1u64 << 40) as f64;
    let mut check = 0.;
    let mut sign = 1.;
    let mut last_result: f64 = 0.;
    loop {
        let result = evaluate("root".to_string(), monkeys, Some(check));
        // println!("Checking {}, result = {}", check, result);
        if result == 0. {
            return check as i64;
        }
        if last_result != 0. && result.abs() > last_result.abs() {
            sign *= -1.;
            mult /= 2.;
        }
        last_result = result;
        check += mult * sign;
    }
}

pub fn step1() {
    let mut monkeys = HashMap::new();
    for rule in read_list("inputs/day21.txt") {
        let mut parts = rule.split_whitespace();
        let rule_id = parts.next().unwrap().trim_end_matches(':').to_string();
        let p1 = parts.next().unwrap();
        let mr: Monkey = match p1.parse::<f64>() {
            Ok(value) => Monkey::Fact(value),
            Err(_) => Monkey::Computation(
                p1.to_string(),
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            ),
        };
        monkeys.insert(rule_id, mr);
    }

    let result = evaluate("root".to_string(), &monkeys, None);
    println!("Root: {}", result);
}

pub fn step2() {
    let mut monkeys = HashMap::new();
    for rule in read_list("inputs/day21.txt") {
        let mut parts = rule.split_whitespace();
        let rule_id = parts.next().unwrap().trim_end_matches(':').to_string();
        let is_root = rule_id == "root";
        let p1 = parts.next().unwrap();
        let mr: Monkey = match p1.parse::<f64>() {
            Ok(value) => Monkey::Fact(value),
            Err(_) => Monkey::Computation(
                p1.to_string(),
                if is_root {
                    parts.next();
                    "=".to_string()
                } else {
                    parts.next().unwrap().to_string()
                },
                parts.next().unwrap().to_string(),
            ),
        };
        monkeys.insert(rule_id, mr);
    }

    let result = solve_for_human(&monkeys);
    println!("Human: {}", result);
}
