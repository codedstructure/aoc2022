use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

struct HeightMap {
    row_count: usize,
    col_count: usize,
    start_point: (usize, usize),
    end_point: (usize, usize),
    elevation: HashMap<(usize, usize), i32>,
}

impl HeightMap {
    fn new(filename: &str) -> Self {
        let mut grid: HashMap<(usize, usize), i32> = HashMap::new();
        let mut col_count = 0;
        for (row_idx, line) in read_list(filename).iter().enumerate() {
            // Read as base 36 and subtract 9 to get a = 1, z = 26
            for (col_idx, height) in line
                .chars()
                .map(|c| c.to_digit(36).unwrap() as i32 - 9)
                .enumerate()
            {
                grid.insert((row_idx, col_idx), height);
            }
            col_count = line.len();
        }
        let mut start_point = (0, 0);
        let mut end_point = (0, 0);
        for (row, line) in read_list("inputs/day12.txt").iter().enumerate() {
            if let Some(col) = line.find('S') {
                start_point = (row, col);
                grid.insert((row, col), 1); // start point 'has elevation a'
            }
            if let Some(col) = line.find('E') {
                end_point = (row, col);
                grid.insert((row, col), 26); // '(E) has elevation z'
            }
        }

        Self {
            row_count: grid.len() / col_count,
            col_count,
            start_point,
            end_point,
            elevation: grid,
        }
    }

    fn neighbours(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let current_elevation = self.elevation[&coord];
        let mut result = vec![];
        if coord.0 > 0 {
            let neighbour = (coord.0 - 1, coord.1);
            if self.elevation[&neighbour] - current_elevation <= 1 {
                result.push(neighbour);
            }
        }
        if coord.0 < self.row_count - 1 {
            let neighbour = (coord.0 + 1, coord.1);
            if self.elevation[&neighbour] - current_elevation <= 1 {
                result.push(neighbour);
            }
        }
        if coord.1 > 0 {
            let neighbour = (coord.0, coord.1 - 1);
            if self.elevation[&neighbour] - current_elevation <= 1 {
                result.push(neighbour);
            }
        }
        if coord.1 < self.col_count - 1 {
            let neighbour = (coord.0, coord.1 + 1);
            if self.elevation[&neighbour] - current_elevation <= 1 {
                result.push(neighbour);
            }
        }
        result
    }
}

// The `State` struct & `shortest_path` functions are derived from
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
//
// Only minor changes have been required / made to support the
// heightmap grid as a graph source, using the `neighbours()` method
// to enumerate edges, each of which has cost 1.

// START CODE DERIVED FROM https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.
fn shortest_path(heightmap: &HeightMap, start_point: (usize, usize)) -> usize {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();

    // We're at `start`, with a zero cost
    dist.insert(start_point, 0);
    heap.push(State {
        cost: 0,
        position: start_point,
    });

    let mut distance = usize::MAX;
    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == heightmap.end_point {
            distance = cost;
            break;
        }
        // Important as we may have already found a better way
        if let Some(candidate_dist) = dist.get(&position) {
            if cost > *candidate_dist {
                continue;
            }
        }
        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in heightmap.neighbours(position) {
            // All 'edges' here have cost 1.
            let next = State {
                cost: cost + 1,
                position: edge,
            };
            // If so, add it to the frontier and continue
            if next.cost < *dist.entry(next.position).or_insert(usize::MAX) {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
            }
        }
    }
    distance
}
// END CODE DERIVED FROM https://doc.rust-lang.org/std/collections/binary_heap/index.html

pub fn step1() {
    let heightmap = HeightMap::new("inputs/day12.txt");
    let cost = shortest_path(&heightmap, heightmap.start_point);
    println!("Distance to target: {}", cost);
}

pub fn step2() {
    let mut heightmap = HeightMap::new("inputs/day12.txt");

    // Set start point back to value for 'a'
    heightmap.elevation.insert(heightmap.start_point, 1);

    let candidates: Vec<_> = heightmap
        .elevation
        .iter()
        .filter(|coord| heightmap.elevation[coord.0] == 1)
        .map(|p| *p.0)
        .collect();

    let mut min_route = usize::MAX;

    for start_point in candidates {
        let cost = shortest_path(&heightmap, start_point);
        if cost < min_route {
            min_route = cost;
        }
    }

    println!("Minimum scenic route length: {}", min_route);
}
