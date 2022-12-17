use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn read_chars(filename: &str) -> Vec<char> {
    let f = File::open(filename).expect("Could not read file");
    let mut buffer = String::new();
    BufReader::new(f).read_to_string(&mut buffer).unwrap();
    return buffer.trim().chars().collect();
}

fn collide(chamber: &[u8], falling: &[u8], offset: usize) -> bool {
    for (idx, row) in falling.iter().enumerate() {
        if (chamber[offset + idx] & row) != 0 {
            return true;
        };
    }
    false
}

#[allow(dead_code)]
fn display(chamber: &[u8]) {
    for row in chamber.iter().rev() {
        println!("{:07b}", row);
    }
    println!("-------\n");
}

pub fn step1() {
    let wind = read_chars("inputs/day17.txt");
    // Chamber represented as a vector of u8s where the high bit
    // should always be zero. chamber[0] is row immediately above
    // the ground.
    let mut chamber: Vec<u8> = vec![];
    let mut wind_idx = 0;
    let mut rock_type = 0;
    for _ in 0..2022 {
        let mut falling: Vec<u8> = vec![];

        match rock_type {
            0 => {
                falling.push(0b0011110);
            }
            1 => {
                falling.push(0b0001000);
                falling.push(0b0011100);
                falling.push(0b0001000);
            }
            2 => {
                // Note: this is opposite order to the shape -
                // lowest row is pushed first.
                falling.push(0b0011100);
                falling.push(0b0000100);
                falling.push(0b0000100);
            }
            3 => {
                falling.push(0b0010000);
                falling.push(0b0010000);
                falling.push(0b0010000);
                falling.push(0b0010000);
            }
            4 => {
                falling.push(0b0011000);
                falling.push(0b0011000);
            }
            _ => {
                panic!("Invalid rock type")
            }
        }

        // Provide some extra air in our column to avoid special casing
        chamber.extend(vec![0; falling.len() + 3]);

        // Current 'height' of the *bottom* of the falling rock
        let mut offset = chamber.len() - falling.len();

        loop {
            // Get blown by the wind
            let mut move_dir = 0;
            if wind[wind_idx] == '<' {
                if !falling.iter().any(|x| x & 64 != 0) {
                    move_dir = -1;
                }
            } else if !falling.iter().any(|x| x & 1 != 0) {
                move_dir = 1;
            }
            if move_dir == -1 {
                let candidate: Vec<u8> = falling.iter().map(|x| x << 1).collect();
                if !collide(&chamber, &candidate, offset) {
                    falling = candidate;
                    //println!("Pushed left by wind");
                }
            } else if move_dir == 1 {
                let candidate: Vec<u8> = falling.iter().map(|x| x >> 1).collect();
                if !collide(&chamber, &candidate, offset) {
                    falling = candidate;
                    //println!("Pushed right by wind");
                }
            } else {
                //println!("Can't push anymore");
            }
            //display(&falling);
            wind_idx = (wind_idx + 1) % wind.len();

            if offset == 0 || collide(&chamber, &falling, offset - 1) {
                break;
            }
            offset -= 1;
        }

        // Merge the falling rock into the solidness below.
        for (idx, row) in falling.iter().enumerate() {
            chamber[offset + idx] |= row;
        }

        // Vacuum up excess air at the top of the chamber
        // Needed to get clear height of the current solid lava
        while chamber.last().unwrap_or(&1) == &0 {
            chamber.pop();
        }

        //println!("Resulting state:");
        //display(&chamber);

        rock_type = (rock_type + 1) % 5;
    }

    //println!("{:?}", wind);
    println!("Height: {}", chamber.len());
}

pub fn step2() {}
