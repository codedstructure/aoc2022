use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

fn calc_score(line: &str) -> i32 {
    let opponent_move = match &line[0..1] {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Invalid opponent move"),
    };
    let our_move = match &line[2..3] {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Invalid our move"),
    };

    let round_result = match our_move - opponent_move {
        0 => 3,  // draw
        1 => 6,  // one ahead in RPS => win
        -1 => 0, // one behind => loss
        2 => 0,  // two ahead => one behind => loss
        -2 => 6, // two behind => one ahead => win
        _ => panic!("Invalid result"),
    };

    our_move + round_result
}

pub fn step1() {
    // Total score of all RPS rounds
    let mut score = 0;
    for line in read_list("inputs/day02.txt") {
        score += calc_score(&line)
    }
    println!("Total score: {}", score);
}

fn calc_move_and_score(line: &str) -> i32 {
    let opponent_move = match &line[0..1] {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Invalid opponent move"),
    };
    let desired_outcome = match &line[2..3] {
        "X" => 0, // lose
        "Y" => 3, // draw
        "Z" => 6, // win
        _ => panic!("Invalid outcome"),
    };

    let our_move = match desired_outcome {
        // lose: one behind
        0 => {
            if opponent_move == 1 {
                3
            } else {
                opponent_move - 1
            }
        }
        // draw: copy opponent
        3 => opponent_move,
        // win: one ahead
        6 => {
            if opponent_move == 3 {
                1
            } else {
                opponent_move + 1
            }
        }
        _ => panic!("Invalid outcome"),
    };

    our_move + desired_outcome
}

pub fn step2() {
    // Total score of all RPS rounds
    let mut score = 0;
    for line in read_list("inputs/day02.txt") {
        score += calc_move_and_score(&line)
    }
    println!("Total score: {}", score);
}
