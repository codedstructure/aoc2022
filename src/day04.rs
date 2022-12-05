use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

fn task_range(task_spec: &str) -> HashSet<i32> {
    // Convert "4-7" into a set of {4,5,6,7}
    let mut task_spec_split = task_spec.split('-');
    let start: i32 = task_spec_split.next().unwrap().parse().unwrap();
    let end: i32 = task_spec_split.next().unwrap().parse().unwrap();
    // RUST: I suspect there's a better way of doing this?
    let mut result = HashSet::<i32>::new();
    for value in start..=end {
        result.insert(value);
    }
    result
}

fn full_overlap(a: &str, b: &str) -> bool {
    let a_set = task_range(a);
    let b_set = task_range(b);
    a_set.is_superset(&b_set) || b_set.is_superset(&a_set)
}

pub fn step1() {
    // Number of elf pairs with fully overlapping task spec
    let mut count = 0;
    for line in read_list("inputs/day04.txt") {
        let mut elf_tasks = line.split(',');
        let elf1_task_spec = elf_tasks.next().unwrap();
        let elf2_task_spec = elf_tasks.next().unwrap();
        if full_overlap(elf1_task_spec, elf2_task_spec) {
            count += 1;
        }
    }
    println!("Count of fully overlapping elf-pair tasks: {}", count);
}

fn any_overlap(a: &str, b: &str) -> bool {
    let a_set = task_range(a);
    let b_set = task_range(b);
    !a_set.is_disjoint(&b_set)
}

pub fn step2() {
    // Number of elf pairs with any overlap in task spec
    let mut count = 0;
    for line in read_list("inputs/day04.txt") {
        let mut elf_tasks = line.split(',');
        let elf1_task_spec = elf_tasks.next().unwrap();
        let elf2_task_spec = elf_tasks.next().unwrap();
        if any_overlap(elf1_task_spec, elf2_task_spec) {
            count += 1;
        }
    }
    println!("Count of overlapping elf-pair tasks: {}", count);
}
