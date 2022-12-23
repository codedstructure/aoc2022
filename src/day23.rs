use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Clone, Copy)]
struct Elf {
    position: (i32, i32),
    // Direction: 0=N, 1=S, 2=W, 3=E
    next_dir: i32,
}

impl Elf {
    fn new(position: (i32, i32)) -> Self {
        Elf {
            position,
            next_dir: 0,
        }
    }

    fn alone(&self, grove: &HashMap<(i32, i32), Elf>) -> bool {
        !(grove.contains_key(&(self.position.0 - 1, self.position.1 - 1))
            || grove.contains_key(&(self.position.0 - 1, self.position.1))
            || grove.contains_key(&(self.position.0 - 1, self.position.1 + 1))
            || grove.contains_key(&(self.position.0, self.position.1 - 1))
            || grove.contains_key(&(self.position.0, self.position.1 + 1))
            || grove.contains_key(&(self.position.0 + 1, self.position.1 - 1))
            || grove.contains_key(&(self.position.0 + 1, self.position.1))
            || grove.contains_key(&(self.position.0 + 1, self.position.1 + 1)))
    }

    fn move_pos(&self, dir: i32) -> (i32, i32) {
        match dir {
            0 => (self.position.0 - 1, self.position.1), // North
            1 => (self.position.0 + 1, self.position.1), // South
            2 => (self.position.0, self.position.1 - 1), // West
            3 => (self.position.0, self.position.1 + 1), // East
            _ => panic!("Invalid direction"),
        }
    }

    fn can_move(&self, dir: i32, grove: &HashMap<(i32, i32), Elf>) -> bool {
        match dir {
            0 => {
                !(grove.contains_key(&(self.position.0 - 1, self.position.1 - 1))
                    || grove.contains_key(&(self.position.0 - 1, self.position.1))
                    || grove.contains_key(&(self.position.0 - 1, self.position.1 + 1)))
            }
            1 => {
                !(grove.contains_key(&(self.position.0 + 1, self.position.1 - 1))
                    || grove.contains_key(&(self.position.0 + 1, self.position.1))
                    || grove.contains_key(&(self.position.0 + 1, self.position.1 + 1)))
            }
            2 => {
                !(grove.contains_key(&(self.position.0 - 1, self.position.1 - 1))
                    || grove.contains_key(&(self.position.0, self.position.1 - 1))
                    || grove.contains_key(&(self.position.0 + 1, self.position.1 - 1)))
            }
            3 => {
                !(grove.contains_key(&(self.position.0 - 1, self.position.1 + 1))
                    || grove.contains_key(&(self.position.0, self.position.1 + 1))
                    || grove.contains_key(&(self.position.0 + 1, self.position.1 + 1)))
            }
            _ => panic!("Invalid direction"),
        }
    }

    fn proposed_move(&self, grove: &HashMap<(i32, i32), Elf>) -> (i32, i32) {
        if self.alone(grove) {
            return self.position;
        }
        let mut check_dir = self.next_dir;
        for _ in 0..4 {
            if self.can_move(check_dir, grove) {
                return self.move_pos(check_dir);
            }
            check_dir = (check_dir + 1) % 4;
        }
        self.position
    }

    fn update(&mut self, pos: (i32, i32)) {
        self.position = pos;
        self.next_dir = (self.next_dir + 1) % 4;
    }
}

#[derive(Debug)]
struct Grove {
    elves: HashMap<(i32, i32), Elf>,
}

impl Grove {
    fn new(filename: &str) -> Self {
        let mut elves = HashMap::new();

        for (row_idx, line) in read_list(filename).iter().enumerate() {
            let row_idx = row_idx as i32;
            for (col_idx, char) in line.chars().enumerate() {
                let col_idx = col_idx as i32;
                if char == '#' {
                    elves.insert((row_idx, col_idx), Elf::new((row_idx, col_idx)));
                }
            }
        }
        Self { elves }
    }

    fn round(&mut self) -> bool {
        let mut proposed: HashMap<(i32, i32), i32> = HashMap::new();
        for elf in self.elves.values() {
            if !elf.alone(&self.elves) {
                let maybe_next_pos = elf.proposed_move(&self.elves);
                *proposed.entry(maybe_next_pos).or_insert(0) += 1;
            }
        }

        if proposed.is_empty() {
            // Finished!
            return true;
        }

        let mut updated_elves: HashMap<(i32, i32), Elf> = HashMap::new();
        for elf in self.elves.values().clone() {
            let proposal = elf.proposed_move(&self.elves);
            if proposed.get(&proposal) == Some(&1) {
                updated_elves.insert(proposal, *elf);
            } else {
                updated_elves.insert(elf.position, *elf);
            }
        }

        for (pos, elf) in &mut updated_elves {
            elf.update(*pos);
        }

        self.elves = updated_elves;
        false // not finished!
    }

    fn extents(&self) -> ((i32, i32), (i32, i32)) {
        let mut min_row = i32::MAX;
        let mut max_row = i32::MIN;
        let mut min_col = i32::MAX;
        let mut max_col = i32::MIN;
        for elf in self.elves.values() {
            if elf.position.0 < min_row {
                min_row = elf.position.0;
            }
            if elf.position.0 > max_row {
                max_row = elf.position.0;
            }
            if elf.position.1 < min_col {
                min_col = elf.position.1;
            }
            if elf.position.1 > max_col {
                max_col = elf.position.1;
            }
        }
        ((min_row, max_row), (min_col, max_col))
    }

    #[allow(dead_code)]
    fn display(&self) {
        let ((min_row, max_row), (min_col, max_col)) = self.extents();
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if self.elves.contains_key(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn empty_ground(&self) -> i32 {
        let ((min_row, max_row), (min_col, max_col)) = self.extents();
        let rows = (max_row - min_row) + 1;
        let cols = (max_col - min_col) + 1;
        let area = rows * cols;
        area - self.elves.len() as i32
    }
}

pub fn step1() {
    let mut grove = Grove::new("inputs/day23.txt");

    for _ in 0..10 {
        grove.round();
    }

    println!("Empty ground: {}", grove.empty_ground());
}
pub fn step2() {
    let mut grove = Grove::new("inputs/day23.txt");

    let mut count = 1; // Wouldn't do to have an off-by-one answer...
    while !grove.round() {
        count += 1;
    }

    println!("Rounds till finished: {}", count);
}
