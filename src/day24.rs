// Blizzard Basin
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Clone, Copy)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

struct Maze {
    blizzards: Vec<(i32, i32, Blizzard)>,
    width: i32,
    height: i32,
}

impl Maze {
    fn new(filename: &str) -> Self {
        let mut blizzards: Vec<(i32, i32, Blizzard)> = vec![];
        let mut width = 0;
        let mut height = 0;
        for (row_idx, line) in read_list(filename).iter().enumerate() {
            let row_idx = row_idx as i32 - 1;
            if row_idx > height {
                height = row_idx;
            }
            for (col_idx, c) in line.chars().enumerate() {
                let col_idx = col_idx as i32 - 1;
                if col_idx > width {
                    width = col_idx;
                }
                if let Some(blizzard) = match c {
                    '<' => Some(Blizzard::Left),
                    '>' => Some(Blizzard::Right),
                    '^' => Some(Blizzard::Up),
                    'v' => Some(Blizzard::Down),
                    _ => None,
                } {
                    blizzards.push((row_idx, col_idx, blizzard));
                }
            }
        }

        Self {
            blizzards,
            width,
            height,
        }
    }

    fn advance(&self, blizzards: &[(i32, i32, Blizzard)]) -> Vec<(i32, i32, Blizzard)> {
        let mut new_blizzards = vec![];

        for (row, col, dir) in blizzards.iter().copied() {
            match dir {
                Blizzard::Left => {
                    new_blizzards.push((row, (col + self.width - 1) % self.width, dir));
                }
                Blizzard::Right => {
                    new_blizzards.push((row, (col + 1) % self.width, dir));
                }
                Blizzard::Up => {
                    new_blizzards.push(((row + self.height - 1) % self.height, col, dir));
                }
                Blizzard::Down => {
                    new_blizzards.push(((row + 1) % self.height, col, dir));
                }
            }
        }

        new_blizzards
    }

    fn blizzard_at(&self, pos: (i32, i32), blizzards: &[(i32, i32, Blizzard)]) -> bool {
        blizzards.iter().any(|(r, c, _)| (*r, *c) == pos)
    }

    fn move_options(&self, pos: (i32, i32), blizzards: &[(i32, i32, Blizzard)]) -> Vec<(i32, i32)> {
        let mut options = vec![];
        // Move right
        if pos.0 >= 0 && pos.1 < self.width - 1 {
            let candidate = (pos.0, pos.1 + 1);
            if !self.blizzard_at(candidate, blizzards) {
                options.push(candidate);
            }
        }
        // Move down
        if pos.0 < self.height - 1 || pos.1 == self.width - 1 {
            let candidate = (pos.0 + 1, pos.1);
            if !self.blizzard_at(candidate, blizzards) {
                options.push(candidate);
            }
        }
        // Stay still
        if (pos.0 >= 0 || pos == (-1, 0) || pos == (self.height, self.width - 1))
            && !self.blizzard_at(pos, blizzards)
        {
            options.push(pos);
        }
        // Move up
        if pos.0 > 0 || pos == (0, 0) {
            let candidate = (pos.0 - 1, pos.1);
            if !self.blizzard_at(candidate, blizzards) {
                options.push(candidate);
            }
        }
        // Move left
        if pos.0 >= 0 && pos.0 < self.height && pos.1 > 0 {
            let candidate = (pos.0, pos.1 - 1);
            if !self.blizzard_at(candidate, blizzards) {
                options.push(candidate);
            }
        }
        options
    }

    fn solve(&self, targets: &[(i32, i32)]) -> i32 {
        let mut minutes = 0;
        let mut positions = vec![(-1, 0)];
        let mut blizzards = self.blizzards.clone();

        let mut target_idx = 0;

        loop {
            minutes += 1;
            let mut next_positions = HashSet::new();
            blizzards = self.advance(&blizzards);
            let b = blizzards.clone();
            for pos in &positions {
                for option in self.move_options(*pos, &b) {
                    next_positions.insert(option);
                }
            }
            for b in &blizzards {
                if next_positions.contains(&(b.0, b.1)) {
                    next_positions.remove(&(b.0, b.1));
                }
            }
            if next_positions.contains(&targets[target_idx]) {
                next_positions.clear();
                next_positions.insert(targets[target_idx]);
                //println!("Reached target {} in {} minutes", target_idx, minutes);
                target_idx += 1;
                if target_idx == targets.len() {
                    break;
                }
            }
            positions = next_positions.into_iter().collect();
        }
        minutes
    }
}

pub fn step1() {
    let maze = Maze::new("inputs/day24.txt");

    let targets = [(maze.height, maze.width - 1)];
    println!("Maze escape time: {}", maze.solve(&targets));
}

pub fn step2() {
    let maze = Maze::new("inputs/day24.txt");

    let targets = [
        (maze.height, maze.width - 1),
        (-1, 0),
        (maze.height, maze.width - 1),
    ];
    println!(
        "Maze escape there, back again, and there again: {}",
        maze.solve(&targets)
    );
}
