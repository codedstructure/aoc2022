use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    position: Coord,
    closest_beacon: Coord,
}

impl Sensor {
    fn possible_beacon(&self, b: Coord) -> bool {
        let min_dist = (self.closest_beacon.x - self.position.x).abs()
            + (self.closest_beacon.y - self.position.y).abs();
        // Could there be a beacon at the given coord - i.e. is it further than min_dist?
        let b_dist = (b.x - self.position.x).abs() + (b.y - self.position.y).abs();
        b_dist > min_dist
    }

    fn boundary(&self) -> Vec<Coord> {
        let mut result = vec![];
        let min_dist = (self.closest_beacon.x - self.position.x).abs()
            + (self.closest_beacon.y - self.position.y).abs();

        for offset in 0..=(min_dist + 1) {
            result.push(Coord {
                x: self.position.x + offset,
                y: self.position.y + (1 + min_dist - offset),
            });
            result.push(Coord {
                x: self.position.x + offset,
                y: self.position.y - (1 + min_dist - offset),
            });
            result.push(Coord {
                x: self.position.x - offset,
                y: self.position.y + (1 + min_dist - offset),
            });
            result.push(Coord {
                x: self.position.x - offset,
                y: self.position.y - (1 + min_dist - offset),
            });
        }

        result
    }
}

struct SensorMap {
    sensors: Vec<Sensor>,
    beacons: HashSet<Coord>,
}

impl SensorMap {
    fn new(filename: &str) -> Self {
        let mut sensors: Vec<Sensor> = vec![];
        let mut beacons: HashSet<Coord> = HashSet::new();
        for line in read_list(filename) {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let s_x: i32 = parts[2]
                .split('=')
                .nth(1)
                .unwrap()
                .strip_suffix(',')
                .unwrap()
                .parse()
                .unwrap();
            let s_y: i32 = parts[3]
                .split('=')
                .nth(1)
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse()
                .unwrap();
            let b_x: i32 = parts[8]
                .split('=')
                .nth(1)
                .unwrap()
                .strip_suffix(',')
                .unwrap()
                .parse()
                .unwrap();
            let b_y: i32 = parts[9].split('=').nth(1).unwrap().parse().unwrap();

            sensors.push(Sensor {
                position: Coord { x: s_x, y: s_y },
                closest_beacon: Coord { x: b_x, y: b_y },
            });
            beacons.insert(Coord { x: b_x, y: b_y });
        }

        Self { sensors, beacons }
    }

    fn possible_beacon(&self, c: Coord) -> bool {
        for s in &self.sensors {
            if !s.possible_beacon(c) && !self.beacons.contains(&c) {
                return false;
            }
        }
        true
    }
}

pub fn step1() {
    let sensor_map = SensorMap::new("inputs/day15.txt");

    let mut beaconless_count = 0;
    for pos in -10000000..10000000 {
        if !sensor_map.possible_beacon(Coord { x: pos, y: 2000000 }) {
            beaconless_count += 1;
        }
    }

    println!("{}", beaconless_count);
}

pub fn step2() {
    let sensor_map = SensorMap::new("inputs/day15.txt");

    // If there is only a single position for the distress beacon, it must be
    // adjacent to a 'no-go' area, therefore must be on the 'boundary' one
    // beyond min-dist of any sensor. Collect all the boundaries together,
    // then traverse this checking for possible beacon positions.

    // Note we could have (a tiny proportion of) duplicates in here, but a
    // Vec is much faster than a set, and this doesn't affect correctness.
    let mut full_boundary: Vec<Coord> = vec![];
    for s in &sensor_map.sensors {
        full_boundary.extend(s.boundary().iter());
    }

    for pos in full_boundary {
        if pos.x < 0 || pos.y < 0 || pos.x > 4000000 || pos.y > 4000000 {
            continue;
        }
        if sensor_map.possible_beacon(pos) {
            println!("Distress beacon at {:?}", pos);
            println!(
                "Tuning frequency: {}",
                pos.x as i64 * 4000000 + pos.y as i64
            );
            break;
        }
    }
}
