use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

impl State {
    fn new() -> Self {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1, // Our starting resource!
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore: i32,
    clay_robot_ore: i32,
    obsidian_robot_ore: i32,
    obsidian_robot_clay: i32,
    geode_robot_ore: i32,
    geode_robot_obsidian: i32,

    cache: HashMap<(State, i32), i32>,
}

impl Blueprint {
    fn new(line: &str) -> Self {
        let mut parts = line.split_whitespace();

        Self {
            ore_robot_ore: parts.nth(6).unwrap().parse().unwrap(),
            clay_robot_ore: parts.nth(5).unwrap().parse().unwrap(),
            obsidian_robot_ore: parts.nth(5).unwrap().parse().unwrap(),
            obsidian_robot_clay: parts.nth(2).unwrap().parse().unwrap(),
            geode_robot_ore: parts.nth(5).unwrap().parse().unwrap(),
            geode_robot_obsidian: parts.nth(2).unwrap().parse().unwrap(),
            cache: HashMap::new(),
        }
    }

    fn recurse(&mut self, state: State, remain: i32) -> i32 {
        //println!("Remain: {}. State: {:?}", remain, state);
        if remain == 0 {
            return state.geodes;
        }

        // First optimisation: caching. This was sufficient to get part 1
        // in a couple of minutes.
        if let Some(cached_result) = self.cache.get(&(state, remain)) {
            return *cached_result;
        }

        let mut next_state = state;

        next_state.ore += state.ore_robots;
        next_state.clay += state.clay_robots;
        next_state.obsidian += state.obsidian_robots;
        next_state.geodes += state.geode_robots;

        // Second optimisation: we never need more robots (i.e. new resources)
        // than enough to build a (any) new robot each minute.
        let max_robot_ore = *vec![
            self.ore_robot_ore,
            self.clay_robot_ore,
            self.obsidian_robot_ore,
            self.geode_robot_ore,
        ]
        .iter()
        .max()
        .unwrap();
        let max_robot_clay = self.obsidian_robot_clay;
        let max_robot_obsidian = self.geode_robot_obsidian;

        let mut best_option = 0;
        // Option 1: build an ore robot
        if state.ore >= self.ore_robot_ore && state.ore_robots < max_robot_ore {
            let mut next_state = next_state;
            next_state.ore -= self.ore_robot_ore;
            next_state.ore_robots += 1;
            let opt = self.recurse(next_state, remain - 1);
            if opt > best_option {
                best_option = opt;
            }
        }
        // Option 2: build a clay robot
        if state.ore >= self.clay_robot_ore && state.clay_robots < max_robot_clay {
            let mut next_state = next_state;
            next_state.ore -= self.clay_robot_ore;
            next_state.clay_robots += 1;
            let opt = self.recurse(next_state, remain - 1);
            if opt > best_option {
                best_option = opt;
            }
        }
        // Option 3: build an obsidian robot
        if state.ore >= self.obsidian_robot_ore
            && state.clay >= self.obsidian_robot_clay
            && state.obsidian_robots < max_robot_obsidian
        {
            let mut next_state = next_state;
            next_state.ore -= self.obsidian_robot_ore;
            next_state.clay -= self.obsidian_robot_clay;
            next_state.obsidian_robots += 1;
            let opt = self.recurse(next_state, remain - 1);
            if opt > best_option {
                best_option = opt;
            }
        }
        // Option 4: build a geode robot.
        if state.ore >= self.geode_robot_ore && state.obsidian >= self.geode_robot_obsidian {
            let mut next_state = next_state;
            next_state.ore -= self.geode_robot_ore;
            next_state.obsidian -= self.geode_robot_obsidian;
            next_state.geode_robots += 1;
            let opt = self.recurse(next_state, remain - 1);
            if opt > best_option {
                best_option = opt;
            }
        }
        // Option 5 - don't build anything, just accumulate resource.
        let opt = self.recurse(next_state, remain - 1);
        if opt > best_option {
            best_option = opt;
        }

        // A limit on inserting cache entries to prevent OOM during part 2...
        if remain > 4 {
            self.cache.insert((state, remain), best_option);
        }

        best_option
    }

    fn geode_count(&mut self, minutes: i32) -> i32 {
        let state = State::new();
        self.recurse(state, minutes)
    }
}

pub fn step1() {
    let mut blueprints = vec![];
    for line in read_list("inputs/day19.txt") {
        blueprints.push(Blueprint::new(&line));
    }

    //println!("Blueprints: {:?}", blueprints);

    let mut total = 0;
    for (idx, b) in blueprints.iter_mut().enumerate() {
        let bp_idx = idx as i32 + 1; // 1-based
        let geodes = b.geode_count(24);
        println!("Blueprint {}: max geode count: {}", bp_idx, geodes);
        total += geodes * bp_idx;
    }
    println!("Total quality level: {}", total);
}

pub fn step2() {
    let mut blueprints = vec![];
    for line in read_list("inputs/day19.txt").iter().take(3) {
        blueprints.push(Blueprint::new(line));
    }
    //println!("Blueprints: {:?}", blueprints);

    let mut product = 1;
    for (idx, b) in blueprints.iter_mut().enumerate() {
        let bp_idx = idx as i32 + 1; // 1-based
        let geodes = b.geode_count(32);
        println!("Blueprint {}: max geode count: {}", bp_idx, geodes);
        product *= geodes;
    }
    println!("Geode product: {}", product);
}
