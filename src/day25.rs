// Full of Hot Air
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

fn from_snafu(s: &str) -> i64 {
    let mut total = 0;

    let mut placevalue = 1;
    for c in s.chars().into_iter().rev() {
        match c {
            '1' => {
                total += placevalue;
            }
            '2' => {
                total += 2 * placevalue;
            }
            '=' => {
                total -= 2 * placevalue;
            }
            '-' => {
                total -= placevalue;
            }
            '0' => {}
            _ => panic!("Unknown digit"),
        }
        placevalue *= 5;
    }
    total
}

pub fn to_snafu(v: i64) -> String {
    let mut result = vec![];

    let mut value = v;
    while value > 0 {
        match value % 5 {
            0 => result.push('0'),
            1 => result.push('1'),
            2 => result.push('2'),
            3 => {
                result.push('=');
                value += 6
            }
            4 => {
                result.push('-');
                value += 4
            }
            _ => panic!("Bad modulus"),
        }
        value /= 5;
    }
    result.reverse();
    String::from_iter(result.iter())
}

pub fn step1() {
    let result: i64 = read_list("inputs/day25.txt")
        .iter()
        .map(|x| from_snafu(x))
        .sum();
    println!("result (decimal): {}", result);
    println!("result (snafu): {}", to_snafu(result));
}
pub fn step2() {}
