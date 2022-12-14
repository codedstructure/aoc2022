use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

fn lex(a: &str) -> Vec<String> {
    // Return vector of elements of the list
    //println!("Lex query: {:?}", a);
    let mut result = vec![];
    let mut pending = vec![];
    let mut depth = 0;
    for c in a.chars() {
        if c == '[' {
            depth += 1;
        }
        if depth > 1 {
            pending.push(c);
        } else if c == ',' || c == ']' {
            let s: String = pending.iter().collect();
            pending.clear();
            result.push(s);
        } else if c != '[' {
            pending.push(c);
        }
        if c == ']' {
            depth -= 1;
        }
    }
    //println!("Lex result: {:?}", result);
    result
}

fn ordered(a: &str, b: &str) -> Ordering {
    //println!("Comparing {:?} {:?}", a, b);
    let a_int_opt: Result<i32, _> = a.parse();
    let b_int_opt: Result<i32, _> = b.parse();

    // RUST: would like to use an if-let chain here (eRFC2497) but not yet stable.
    //if let Ok(a_int) = a_int_opt && let Ok(b_int) = b_int_opt { ... }
    if a_int_opt.is_ok() && b_int_opt.is_ok() {
        // Two integers
        a_int_opt.ok().cmp(&b_int_opt.ok())
    } else if a_int_opt.is_err() && b_int_opt.is_err() {
        // Two lists
        let a_vec = lex(a);
        let b_vec = lex(b);
        let mut a_elements = a_vec.iter();
        let mut b_elements = b_vec.iter();
        loop {
            let a_el = a_elements.next();
            let b_el = b_elements.next();
            if a_el.is_none() && b_el.is_none() {
                return Ordering::Equal;
            } else if a_el.is_none() {
                return Ordering::Less;
            } else if b_el.is_none() {
                return Ordering::Greater;
            } else {
                // RUST: clippy would like the conditions checked in a different order,
                // which would be fine if we could have an if-let chain and check
                // both were Ok() first...
                #[allow(clippy::unnecessary_unwrap)]
                let el_cmp = ordered(a_el.unwrap(), b_el.unwrap());
                if el_cmp != Ordering::Equal {
                    return el_cmp;
                };
            }
        }
    } else if a_int_opt.is_ok() || b_int_opt.is_ok() {
        // One list, one int. Convert the int element to a list and try again
        if a_int_opt.is_err() {
            ordered(a, &format!("[{}]", b))
        } else {
            ordered(&format!("[{}]", a), b)
        }
    } else {
        panic!("Invalid comparison between {:?} and {:?}", a, b);
    }
}

pub fn step1() {
    let mut pairs: Vec<(String, String)> = vec![];
    let mut first: Option<String> = None;
    let mut second: Option<String> = None;
    for line in read_list("inputs/day13.txt") {
        if line.is_empty() {
            pairs.push((first.unwrap(), second.unwrap()));
            first = None;
            second = None;
        } else if first.is_some() {
            second = Some(line);
        } else {
            assert!(first.is_none());
            first = Some(line);
        }
    }

    let mut total = 0;
    for (pair_idx, (a, b)) in pairs.iter().enumerate() {
        if ordered(a, b) == Ordering::Less {
            //println!("Right order");
            total += pair_idx + 1; // zero-based -> one-based.
        }
        //println!();
    }
    println!("Sum of ordered indices: {}", total);
}

pub fn step2() {
    let mut packets = vec![];
    for line in read_list("inputs/day13.txt") {
        if !line.is_empty() {
            packets.push(line);
        }
    }

    // Add in the two divider packets
    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());

    packets.sort_by(|a, b| ordered(a, b));

    let mut first_div_idx = 0;
    let mut second_div_idx = 0;
    for (idx, p) in packets.iter().enumerate() {
        if p == "[[2]]" {
            first_div_idx = idx + 1;
        } else if p == "[[6]]" {
            second_div_idx = idx + 1;
        }
    }
    println!("Decoder key: {}", first_div_idx * second_div_idx);
}
