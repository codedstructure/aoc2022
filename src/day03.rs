use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

fn item_value(x: char) -> u8 {
    match x {
        'a'..='z' => x as u8 - b'a' + 1,
        'A'..='Z' => x as u8 - b'A' + 27,
        _ => 0,
    }
}

fn make_set(compartment: &str) -> HashSet<u8> {
    compartment.chars().map(item_value).collect()
}

fn joint_item_value(line: &str) -> i32 {
    let compartment1 = &line[0..(line.len() / 2)];
    let compartment2 = &line[(line.len() / 2)..];

    let c1 = make_set(compartment1);
    let c2 = make_set(compartment2);

    let mut joint_items = c1.intersection(&c2);
    let item = joint_items.next();
    *item.unwrap() as i32
}

pub fn step1() {
    // Total priority of items in both compartments of each rucksack
    let mut score = 0;
    for line in read_list("inputs/day03.txt") {
        score += joint_item_value(&line)
    }
    println!("Total of item priorities: {}", score);
}

fn find_common_item(group: &[String]) -> char {
    // find char which is in all of the entries
    assert!(group.len() == 3);
    let mut common: HashSet<_> = group[0].chars().collect();
    for elf in &group[1..] {
        let elf_items: HashSet<_> = elf.chars().collect();
        // RUST: took me a long time to find the '.copied()' method here...
        common = elf_items.intersection(&common).copied().collect();
    }
    assert!(common.len() == 1);
    *common.iter().next().unwrap()
}

pub fn step2() {
    // Total priority of badges across each group of three elves
    let mut score: i32 = 0;
    for group in read_list("inputs/day03.txt").chunks(3) {
        let elf_group = group;
        let badge_value = find_common_item(elf_group);
        score += item_value(badge_value) as i32;
    }
    println!("Total priority of badges across groups: {}", score);
}
