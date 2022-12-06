use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

pub fn read_chars(filename: &str) -> Vec<char> {
    let f = File::open(filename).expect("Could not read file");
    let mut buffer = String::new();
    BufReader::new(f).read_to_string(&mut buffer).unwrap();
    return buffer.chars().collect();
}

pub fn step1() {
    let stream = read_chars("inputs/day06.txt");
    for start_pos in 4..=stream.len() {
        let mut marker_chars = HashSet::new();
        for offset in 1..=4 {
        marker_chars.insert(stream[start_pos - offset]);
        }

        if marker_chars.len() == 4 {
            println!("Packet found at position {}", start_pos);
            break;
        }
    }
}

pub fn step2() {
    let stream = read_chars("inputs/day06.txt");
    for start_pos in 14..=stream.len() {
        let mut marker_chars = HashSet::new();
        for offset in 1..=14 {
            marker_chars.insert(stream[start_pos - offset]);
        }

        if marker_chars.len() == 14 {
            println!("Message found at position {}", start_pos);
            break;
        }
    }
}