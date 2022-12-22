use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, PartialEq)]
enum CellType {
    Void,
    Wall,
    Empty,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct State {
    direction: Direction,
    position: (i32, i32),
}

impl State {
    fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }
    fn turn_left(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::East => self.direction = Direction::North,
            Direction::South => self.direction = Direction::East,
            Direction::West => self.direction = Direction::South,
        }
    }

    fn value(&self) -> i32 {
        (self.position.0 + 1) * 1000
            + (self.position.1 + 1) * 4
            + (match self.direction {
                Direction::East => 0,
                Direction::South => 1,
                Direction::West => 2,
                Direction::North => 3,
            })
    }
}

#[derive(Debug)]
struct World {
    grid: HashMap<(i32, i32), CellType>,
    state: State,
    rules: Vec<char>,
    is_cube: bool,
}

impl World {
    fn new(filename: &str, is_cube: bool) -> Self {
        let mut grid: HashMap<(i32, i32), CellType> = HashMap::new();
        let mut first_pos = None;
        let lines = read_list(filename);
        let mut rules: String = "".to_string();
        for (row_idx, line) in lines.iter().enumerate() {
            if line.is_empty() {
                rules = lines[row_idx + 1].clone();
                break;
            }
            let row_idx = row_idx as i32;
            for (col_idx, c) in line.chars().enumerate() {
                let col_idx = col_idx as i32;
                match c {
                    ' ' => {
                        grid.insert((row_idx, col_idx), CellType::Void);
                    }
                    '.' => {
                        if first_pos.is_none() {
                            first_pos = Some((row_idx, col_idx));
                        }
                        grid.insert((row_idx, col_idx), CellType::Empty);
                    }
                    '#' => {
                        grid.insert((row_idx, col_idx), CellType::Wall);
                    }
                    _ => panic!("Invalid branch"),
                }
            }
        }
        Self {
            grid,
            state: State {
                direction: Direction::East,
                position: first_pos.unwrap(),
            },
            rules: rules.chars().collect(),
            is_cube,
        }
    }

    fn forward_wrap(&mut self, count: u32) {
        let incr = match self.state.direction {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        };
        for _ in 0..count {
            let mut target_pos = (
                self.state.position.0 + incr.0,
                self.state.position.1 + incr.1,
            );
            let mut target_cell = self.grid.get(&target_pos);
            while target_cell.is_none() || target_cell.unwrap() == &CellType::Void {
                if target_pos.0 < 0 {
                    target_pos.0 = 200;
                }
                if target_pos.0 > 200 {
                    target_pos.0 = -1;
                }
                if target_pos.1 < 0 {
                    target_pos.1 = 150;
                }
                if target_pos.1 > 150 {
                    target_pos.1 = -1;
                }

                target_pos.0 += incr.0;
                target_pos.1 += incr.1;
                target_cell = self.grid.get(&target_pos);
            }
            if target_cell.unwrap() == &CellType::Empty {
                self.state.position = target_pos;
            }
            // otherwise we hit a wall; do nothing.
        }
        //println!("World state: {:?}", self.state);
    }

    fn cube_face(&self, position: (i32, i32)) -> Option<char> {
        // Cube layout (at least for my data):
        //   AB
        //   C
        //  DE
        //  F
        if position.0 < 0 || position.0 > 199 || position.1 < 0 || position.1 > 149 {
            return None;
        }
        if position.0 < 50 {
            if position.1 < 50 {
                None
            } else if position.1 < 100 {
                Some('A')
            } else {
                Some('B')
            }
        } else if position.0 < 100 {
            if position.1 < 50 || position.1 > 99 {
                return None;
            };
            Some('C')
        } else if position.0 < 150 {
            if position.1 > 99 {
                return None;
            };
            if position.1 < 50 {
                Some('D')
            } else {
                Some('E')
            }
        } else {
            if position.1 > 49 {
                return None;
            };
            Some('F')
        }
    }

    fn same_face(&self, position: (i32, i32)) -> bool {
        let my_face = self.cube_face(self.state.position);
        assert!(my_face.is_some());
        my_face == self.cube_face(position)
    }

    fn forward_cube(&mut self, count: u32) {
        // Cube layout (at least for my data):
        //   AB
        //   C
        //  DE
        //  F
        //
        // So interesting transitions are:
        // A & West
        // A & North
        // B & East
        // B & South
        // B & North
        // C & East
        // C & West
        // D & West
        // D & North
        // E & East
        // E & South
        // F & East
        // F & South
        // F & West
        const A_TOP: i32 = 0;
        const A_LEFT: i32 = 50;
        const A_BOTTOM: i32 = 49;

        const B_TOP: i32 = 0;
        const B_LEFT: i32 = 100;
        const B_BOTTOM: i32 = 49;
        const B_RIGHT: i32 = 149;

        const C_TOP: i32 = 50;
        const C_LEFT: i32 = 50;
        const C_RIGHT: i32 = 99;

        const D_TOP: i32 = 100;
        const D_LEFT: i32 = 0;
        const D_BOTTOM: i32 = 149;

        const E_TOP: i32 = 100;
        const E_LEFT: i32 = 50;
        const E_RIGHT: i32 = 99;
        const E_BOTTOM: i32 = 149;

        const F_TOP: i32 = 150;
        const F_LEFT: i32 = 0;
        const F_RIGHT: i32 = 49;
        const F_BOTTOM: i32 = 199;

        for _ in 0..count {
            let incr = match self.state.direction {
                Direction::North => (-1, 0),
                Direction::East => (0, 1),
                Direction::South => (1, 0),
                Direction::West => (0, -1),
            };
            let target_pos = (
                self.state.position.0 + incr.0,
                self.state.position.1 + incr.1,
            );
            // Defaults: nice transitions, such as A->B
            let mut next_pos = target_pos;
            let mut next_dir = self.state.direction;
            if !self.same_face(target_pos) {
                // At this point, we're leaving our current face in
                // state.direction. Do cube wrap-around.
                match self.cube_face(self.state.position).unwrap() {
                    'A' => {
                        let offset = (
                            self.state.position.0 - A_TOP,
                            self.state.position.1 - A_LEFT,
                        );
                        match self.state.direction {
                            Direction::West => {
                                // Moves to D
                                next_dir = Direction::East;
                                next_pos = (D_BOTTOM - offset.0, D_LEFT);
                            }
                            Direction::North => {
                                // Moves to F
                                next_dir = Direction::East;
                                next_pos = (offset.1 + F_TOP, F_LEFT);
                            }
                            _ => {}
                        }
                    }
                    //   AB
                    //   C
                    //  DE
                    //  F
                    'B' => {
                        let offset = (
                            self.state.position.0 - B_TOP,
                            self.state.position.1 - B_LEFT,
                        );
                        match self.state.direction {
                            Direction::East => {
                                // Moves to E, inverted
                                next_dir = Direction::West;
                                next_pos = (E_BOTTOM - offset.0, E_RIGHT);
                            }
                            Direction::South => {
                                // Moves to C
                                next_dir = Direction::West;
                                next_pos = (offset.1 + C_TOP, C_RIGHT);
                            }
                            Direction::North => {
                                // Moves to F, direction unchanged
                                next_pos = (F_BOTTOM, offset.1 + F_LEFT);
                            }
                            _ => {}
                        }
                    }
                    'C' => {
                        let offset = (
                            self.state.position.0 - C_TOP,
                            self.state.position.1 - C_LEFT,
                        );
                        match self.state.direction {
                            Direction::East => {
                                // Moves to B
                                next_dir = Direction::North;
                                next_pos = (B_BOTTOM, offset.0 + B_LEFT);
                            }
                            Direction::West => {
                                // Moves to D
                                next_dir = Direction::South;
                                next_pos = (D_TOP, offset.0 + D_LEFT);
                            }
                            _ => {}
                        }
                    }
                    'D' => {
                        let offset = (
                            self.state.position.0 - D_TOP,
                            self.state.position.1 - D_LEFT,
                        );
                        match self.state.direction {
                            Direction::West => {
                                // Moves to A
                                next_dir = Direction::East;
                                next_pos = (A_BOTTOM - offset.0, A_LEFT);
                            }
                            Direction::North => {
                                // Moves to C
                                next_dir = Direction::East;
                                next_pos = (offset.1 + C_TOP, C_LEFT);
                            }
                            _ => {}
                        }
                    }
                    'E' => {
                        let offset = (
                            self.state.position.0 - E_TOP,
                            self.state.position.1 - E_LEFT,
                        );
                        match self.state.direction {
                            Direction::East => {
                                // Moves to B
                                next_dir = Direction::West;
                                next_pos = (B_BOTTOM - offset.0, B_RIGHT);
                            }
                            Direction::South => {
                                // Moves to F
                                next_dir = Direction::West;
                                next_pos = (offset.1 + F_TOP, F_RIGHT);
                            }
                            _ => {}
                        }
                    }
                    'F' => {
                        let offset = (
                            self.state.position.0 - F_TOP,
                            self.state.position.1 - F_LEFT,
                        );
                        match self.state.direction {
                            Direction::East => {
                                // Moves to E
                                next_dir = Direction::North;
                                next_pos = (E_BOTTOM, offset.0 + E_LEFT);
                            }
                            Direction::South => {
                                // Moves to B, direction unchanged
                                next_pos = (B_TOP, offset.1 + B_LEFT);
                            }
                            Direction::West => {
                                // Moves to A
                                next_dir = Direction::South;
                                next_pos = (A_TOP, offset.0 + A_LEFT);
                            }
                            _ => {}
                        }
                    }
                    _ => panic!("Unkown cube face"),
                }
            }
            if self.grid.get(&next_pos) == Some(&CellType::Wall) {
                // Blocked; no point trying to go forward any further
                break;
            }
            self.state.position = next_pos;
            self.state.direction = next_dir;
        }
    }

    fn forward(&mut self, forward_count: u32) {
        if self.is_cube {
            self.forward_cube(forward_count);
        } else {
            self.forward_wrap(forward_count);
        }
    }

    fn traverse(&mut self) {
        let mut forward_count: u32 = 0;
        for c in &self.rules.clone() {
            match c {
                'L' => {
                    self.forward(forward_count);
                    forward_count = 0;
                    self.state.turn_left()
                }
                'R' => {
                    self.forward(forward_count);
                    forward_count = 0;
                    self.state.turn_right()
                }
                '0'..='9' => {
                    forward_count *= 10;
                    forward_count += c.to_digit(10).unwrap()
                }
                _ => panic!("Unknown rule"),
            }
        }
        self.forward(forward_count);
    }
}

pub fn step1() {
    let mut world = World::new("inputs/day22.txt", false);

    world.traverse();
    println!("Flat world value: {}", world.state.value());
}

pub fn step2() {
    let mut world = World::new("inputs/day22.txt", true);

    world.traverse();
    println!("Cube world value: {}", world.state.value());
}
