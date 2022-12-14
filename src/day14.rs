use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord(i32, i32);

impl From<&str> for Coord {
    fn from(c: &str) -> Self {
        let mut parts = c.split(',');
        Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }
}

impl From<(i32, i32)> for Coord {
    fn from(c: (i32, i32)) -> Self {
        Self(c.0, c.1)
    }
}

fn range(from_v: Coord, to_v: Coord) -> Vec<Coord> {
    let mut result = vec![];
    let mut offset = 0;
    if from_v.0 == to_v.0 {
        loop {
            let step: Coord = (from_v.0, from_v.1 + offset).into();
            result.push(step);
            if step == to_v {
                break;
            }
            offset += (to_v.1 - from_v.1).signum();
        }
    } else if from_v.1 == to_v.1 {
        loop {
            let step: Coord = (from_v.0 + offset, from_v.1).into();
            result.push(step);
            if step == to_v {
                break;
            }
            offset += (to_v.0 - from_v.0).signum();
        }
    } else {
        panic!("Coords must match in one axis")
    }

    result
}

struct Cave {
    grid: HashSet<Coord>,
    max_y: i32,
}

impl Cave {
    fn new(filename: &str, part_2: bool) -> Self {
        let mut grid = HashSet::<Coord>::new();
        let mut max_y = 0;
        for line in read_list(filename) {
            let vertices = line.split(" -> ");
            let mut last_vert = None;
            for vert in vertices {
                if last_vert.is_none() {
                    last_vert = Some(vert);
                }
                let from_v = last_vert.unwrap().into();
                let to_v = vert.into();
                for cell in range(from_v, to_v) {
                    grid.insert(cell);
                    if cell.1 > max_y {
                        max_y = cell.1;
                    }
                }
                last_vert = Some(vert);
            }
        }
        if part_2 {
            // "Infinite" floor two beyond max_y.
            // Since max slope is y = x, can emulate with just max_y either side of the starting x value (500)
            max_y += 2;
            for cell in range((500 - max_y, max_y).into(), (500 + max_y, max_y).into()) {
                grid.insert(cell);
            }
        }
        Self { grid, max_y }
    }

    fn drop_sand(&mut self, start: Coord) -> bool {
        // from Part Two - blocking the entrance also stops things
        if self.grid.contains(&start) {
            return false;
        }

        let mut pos = start;
        loop {
            if pos.1 > self.max_y {
                // did not come to rest
                return false;
            }
            if !self.grid.contains(&(pos.0, pos.1 + 1).into()) {
                // move down one unit
                pos = (pos.0, pos.1 + 1).into();
            } else if !self.grid.contains(&(pos.0 - 1, pos.1 + 1).into()) {
                // move down one and one to the left
                pos = (pos.0 - 1, pos.1 + 1).into();
            } else if !self.grid.contains(&(pos.0 + 1, pos.1 + 1).into()) {
                // move down one and one to the right
                pos = (pos.0 + 1, pos.1 + 1).into();
            } else {
                // sand has come to rest.
                self.grid.insert(pos);
                return true;
            }
        }
    }
}

pub fn step1() {
    let mut cave = Cave::new("inputs/day14.txt", false);
    let mut count = 0;
    while cave.drop_sand(Coord::from((500, 0))) {
        count += 1;
    }

    println!("Number of sand grains: {}", count);
}

pub fn step2() {
    let mut cave = Cave::new("inputs/day14.txt", true);
    let mut count = 0;
    while cave.drop_sand(Coord::from((500, 0))) {
        count += 1;
    }

    println!("Number of sand grains (part 2): {}", count);
}
