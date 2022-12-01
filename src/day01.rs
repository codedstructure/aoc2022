use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub fn read_int_list(filename: &str) -> Result<Vec<Option<i32>>, Error> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f)
        .lines()
        .map(|l| l.expect("Err"))
        .map(|l| l.parse::<i32>().ok())  // None for blank lines
        .collect())
}

pub fn step1() {
    // Sum of largest chunk
    let mut current = 0;
    let mut max_sum = 0;
    for value in read_int_list("inputs/day01.txt").unwrap() {
        if let Some(entry) = value {
            current += entry;
        } else {
            if current > max_sum {
                max_sum = current;
            }
            current = 0;
        }
    }
    println!("Maximum sum: {}", max_sum);
}

pub fn step2() {
    // Sum of top 3 chunks
    let mut current = 0;
    let mut max_sum_1 = 0;
    let mut max_sum_2 = 0;
    let mut max_sum_3 = 0;
    for value in read_int_list("inputs/day01.txt").unwrap() {
        if let Some(entry) = value {
            current += entry;
        } else {
            if current > max_sum_1 {
                max_sum_3 = max_sum_2;
                max_sum_2 = max_sum_1;
                max_sum_1 = current;
            } else if current > max_sum_2 {
                max_sum_3 = max_sum_2;
                max_sum_2 = current;
            } else if current > max_sum_3 {
                max_sum_3 = current;
            }
            current = 0;
        }
    }
    println!("Maximum sum of top 3: {}", max_sum_1 + max_sum_2 + max_sum_3);
}
