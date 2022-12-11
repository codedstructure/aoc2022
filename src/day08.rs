use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

pub fn step1() {
    let mut grid: Vec<Vec<i32>> = vec![];
    for line in read_list("inputs/day08.txt") {
        let row: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        grid.push(row);
    }

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    // Edge trees are visible even at height 0, so ensure max_height starts -1

    // Forward across lines
    for (row_idx, row) in grid.iter().enumerate() {
        let mut max_height = -1;
        for (col_idx, height) in row.iter().enumerate() {
            if height > &max_height {
                visible_trees.insert((row_idx, col_idx));
                max_height = *height;
            }
        }
    }
    // Backward across lines
    for (row_idx, row) in grid.iter().enumerate() {
        let mut max_height = -1;
        for (col_idx, height) in row.iter().enumerate().rev() {
            if height > &max_height {
                visible_trees.insert((row_idx, col_idx));
                max_height = *height;
            }
        }
    }
    // Down each column
    for col_idx in 0..grid[0].len() {
        let mut max_height = -1;
        for row_idx in 0..grid.len() {
            let height = grid[row_idx][col_idx];
            if height > max_height {
                visible_trees.insert((row_idx, col_idx));
                max_height = height;
            }
        }
    }
    // Up each column
    for col_idx in 0..grid[0].len() {
        let mut max_height = -1;
        for row_idx in (0..grid.len()).rev() {
            let height = grid[row_idx][col_idx];
            if height > max_height {
                visible_trees.insert((row_idx, col_idx));
                max_height = height;
            }
        }
    }
    println!("Visible trees: {}", visible_trees.len());
}

fn scenic_score(row_idx: usize, col_idx: usize, grid: &Vec<Vec<i32>>) -> i32 {
    let height = grid[row_idx][col_idx];

    let mut view_dist_right = 0;
    for check_col in (col_idx + 1)..grid[0].len() {
        view_dist_right += 1;
        if grid[row_idx][check_col] >= height {
            break;
        }
    }
    let mut view_dist_left = 0;
    for check_col in (0..col_idx).rev() {
        view_dist_left += 1;
        if grid[row_idx][check_col] >= height {
            break;
        }
    }
    let mut view_dist_up = 0;
    for check_row in (0..row_idx).rev() {
        view_dist_up += 1;
        if grid[check_row][col_idx] >= height {
            break;
        }
    }
    let mut view_dist_down = 0;
    for check_row in (row_idx + 1)..grid.len() {
        view_dist_down += 1;
        if grid[check_row][col_idx] >= height {
            break;
        }
    }

    view_dist_down * view_dist_left * view_dist_right * view_dist_up
}

pub fn step2() {
    let mut grid: Vec<Vec<i32>> = vec![];
    for line in read_list("inputs/day08.txt") {
        let row: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        grid.push(row);
    }

    let mut scenic_score_max = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for col_idx in 0..row.len() {
            let score = scenic_score(row_idx, col_idx, &grid);
            if score > scenic_score_max {
                scenic_score_max = score;
            }
        }
    }

    println!("Max scenic score: {}", scenic_score_max);
}
