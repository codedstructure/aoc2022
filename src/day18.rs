use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

pub fn neighbours(point: u32) -> Vec<u32> {
    let mut result = vec![];
    let x = point >> 16;
    let y = (point & 0x00ff00) >> 8;
    let z = point & 0xff;
    if z == 0 {
        println!("{},{},{}", x, y, z);
    }
    result.push(((x + 1) << 16) + (y << 8) + z);
    result.push(((x - 1) << 16) + (y << 8) + z);
    result.push((x << 16) + ((y + 1) << 8) + z);
    result.push((x << 16) + ((y - 1) << 8) + z);
    result.push((x << 16) + (y << 8) + (z + 1));
    result.push((x << 16) + (y << 8) + (z - 1));
    result
}

pub fn reached_exit(point: u32) -> bool {
    // Inspection of the data shows a minimum coord of 0 in each
    // dimension and max of 19; however we deliberately add 1
    // to avoid potential underflow. Boundary is therefore 0..21.
    let x = point >> 16;
    let y = (point & 0x00ff00) >> 8;
    let z = point & 0xff;
    x == 0 || y == 0 || z == 0 || x == 21 || y == 21 || z == 21
}

pub fn exterior(point: u32, volume: &HashSet<u32>, tried: &mut HashSet<u32>) -> bool {
    // Recursive flood-fill algorithm; see if we can get out
    if reached_exit(point) {
        return true;
    }
    tried.insert(point);

    for n in neighbours(point) {
        if volume.contains(&n) || tried.contains(&n) {
            continue;
        }
        if exterior(n, volume, tried) {
            return true;
        }
    }
    false
}

fn is_exterior(point: u32, volume: &HashSet<u32>) -> bool {
    let mut tried: HashSet<u32> = HashSet::new();
    exterior(point, volume, &mut tried)
}

fn generate_data() -> HashSet<u32> {
    let mut volume: HashSet<u32> = HashSet::new();
    for line in read_list("inputs/day18.txt") {
        let mut parts = line.split(',');
        // Add one to each coordinate, shifting the whole thing without
        // changing it's surface area. This allows use of u32 arithmetic
        // without having to deal with underflow.
        let x: u32 = parts.next().unwrap().parse::<u32>().unwrap() + 1;
        let y: u32 = parts.next().unwrap().parse::<u32>().unwrap() + 1;
        let z: u32 = parts.next().unwrap().parse::<u32>().unwrap() + 1;

        // Urgh, I always forget that << has lower precedence than +.
        let point = (x << 16) + (y << 8) + z;
        volume.insert(point);
    }
    volume
}

pub fn step1() {
    let volume = generate_data();

    let mut surface_area = 0;
    for cell in &volume {
        for n in neighbours(*cell) {
            if !volume.contains(&n) {
                surface_area += 1;
            }
        }
    }
    println!("Surface area: {}", surface_area);
}

pub fn step2() {
    let volume = generate_data();

    let mut surface_area = 0;
    for cell in &volume {
        for n in neighbours(*cell) {
            if !volume.contains(&n) && is_exterior(n, &volume) {
                surface_area += 1;
            }
        }
    }
    println!("Exterior surface area: {}", surface_area);
}
