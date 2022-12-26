use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValveState {
    Opened,
    Closed,
}

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: i32,
    tunnels: HashMap<String, i32>,
}

#[derive(Debug)]
struct Volcano {
    valves: HashMap<String, Valve>,
    distance: HashMap<(String, String), i32>,
}

impl Volcano {
    fn new(filename: &str) -> Self {
        let mut valves = HashMap::new();
        for line in read_list(filename) {
            let mut tunnels = HashMap::new();
            let mut parts = line.split_whitespace();
            let name = parts.nth(1).unwrap().to_string();
            let flow_rate: i32 = parts
                .nth(2)
                .unwrap()
                .trim_start_matches("rate=")
                .trim_end_matches(';')
                .parse()
                .unwrap();
            let _ = parts.nth(3); // advance_by is unstable
            for tunnel in parts {
                let tun_name = tunnel.trim_end_matches(',');
                tunnels.insert(tun_name.to_string(), 1);
            }
            valves.insert(name.clone(), Valve { flow_rate, tunnels });
        }
        Self {
            valves,
            distance: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn render_dot(&self) {
        println!("graph {{");
        println!("overlap=scale");
        println!("layout=neato");
        println!("graph [size=10]");
        println!("edge [dir=none,len=2]");

        let mut edges_seen = HashSet::new();

        for valve in &self.valves {
            if valve.0 == "AA" {
                println!("\"{}\" [color=\"#0000ff\",shape=\"rect\"]", valve.0);
            } else if valve.1.flow_rate != 0 {
                println!(
                    "\"{}\" [color=\"#ff0000\",label=\"{}:{}\"]",
                    valve.0, valve.0, valve.1.flow_rate
                );
            } else {
                println!("\"{}\" [shape=\"none\"]", valve.0);
            }
        }

        for valve in &self.valves {
            for edge in &valve.1.tunnels {
                if !edges_seen.contains(&(valve.0, edge.0))
                    && !edges_seen.contains(&(edge.0, valve.0))
                {
                    edges_seen.insert((valve.0, edge.0));
                    print!("{} -- {} [label=\"{}\"]", valve.0, edge.0, edge.1);
                }
            }
        }
        println!("}}");
    }

    fn neighbours(&self, pos: String) -> Vec<(String, i32)> {
        let mut result = vec![];
        for n in &self.valves[&pos].tunnels {
            result.push((n.0.clone(), *n.1));
        }
        result
    }

    fn remove_pos(&mut self, pos: String) {
        for source in self.neighbours(pos.clone()) {
            for target in self.neighbours(pos.clone()) {
                if source.0 == target.0 {
                    continue;
                }
                self.valves
                    .get_mut(&source.0)
                    .unwrap()
                    .tunnels
                    .insert(target.0.clone(), source.1 + target.1);
                self.valves
                    .get_mut(&target.clone().0)
                    .unwrap()
                    .tunnels
                    .remove(&pos);
            }
            self.valves
                .get_mut(&source.clone().0)
                .unwrap()
                .tunnels
                .remove(&pos);
        }
        self.valves.remove(&pos);
    }

    fn reduce(&mut self) {
        let to_remove: Vec<String> = self
            .valves
            .iter()
            .filter(|v| v.0 != "AA" && v.1.flow_rate == 0)
            .map(|x| x.0.clone())
            .collect();
        for doomed in to_remove {
            self.remove_pos(doomed);
        }
    }

    fn calc_all_pairs_shortest(&mut self) {
        // See https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm
        let mut dist: HashMap<(String, String), i32> = HashMap::new();
        let names: Vec<String> = self.valves.iter().map(|v| v.0.clone()).collect();
        // initialise all pairs to a big number
        for vn_i in &names {
            for vn_j in &names {
                // 'infinity', but use a somewhat lower value to avoid i32 overflow.
                dist.insert((vn_i.clone(), vn_j.clone()), 1000000);
            }
        }
        for v in &self.valves {
            for t in &v.1.tunnels {
                // fill in the known neighbour distances
                dist.insert((v.0.clone(), t.0.clone()), *t.1);
            }
            // Distance to self is null
            dist.insert((v.0.clone(), v.0.clone()), 0);
        }

        for vn_i in &names {
            for vn_j in &names {
                for vn_k in &names {
                    let ik_kj_dist =
                        dist[&(vn_i.clone(), vn_k.clone())] + dist[&(vn_k.clone(), vn_j.clone())];
                    if dist[&(vn_i.clone(), vn_j.clone())] > ik_kj_dist {
                        dist.insert((vn_i.clone(), vn_j.clone()), ik_kj_dist);
                    }
                }
            }
        }

        self.distance = dist;
    }

    fn brute_force(
        &self,
        remain: i32,
        current_pos: String,
        valve_state: HashMap<String, ValveState>,
    ) -> i32 {
        if remain < 0 {
            return 0;
        }
        if valve_state.iter().all(|v| v.1 == &ValveState::Opened) {
            return 0;
        }

        let mut best_pressure = 0;
        for next_check in &valve_state {
            let valve_name = next_check.0;
            let valve_open = valve_state.get(valve_name).unwrap();
            if valve_open == &ValveState::Opened {
                continue;
            }
            let flow_rate = self.valves.get(valve_name).unwrap().flow_rate;
            let mut candidate = valve_state.clone();
            candidate.insert(valve_name.clone(), ValveState::Opened);
            // +1 minute to open the valve...
            let time_cost = self
                .distance
                .get(&(current_pos.clone(), valve_name.clone()))
                .unwrap()
                + 1;
            let extra_pressure = flow_rate * (remain - time_cost);
            let valve_calc = self.brute_force(remain - time_cost, valve_name.clone(), candidate);
            if valve_calc + extra_pressure > best_pressure {
                best_pressure = valve_calc + extra_pressure;
            }
        }
        best_pressure
    }

    fn calculate(&self, remain: i32) -> i32 {
        let mut valve_state: HashMap<String, ValveState> = self
            .valves
            .iter()
            .map(|v| (v.0.clone(), ValveState::Closed))
            .collect();
        valve_state.insert("AA".to_string(), ValveState::Opened);
        self.brute_force(remain, "AA".to_string(), valve_state)
    }

    fn all_partitions(&self) -> Vec<HashSet<String>> {
        let mut result = vec![];
        let valves: Vec<String> = self.valves.iter().map(|v| v.0.clone()).collect();

        // Only need to iterate over len(valves)-1 bits; leave the top bit always
        // clear as inverse is identical a pattern we've had.
        for part_idx in 0..(1 << (valves.len() - 1)) {
            // Heuristic: splitting the work evenly is probably going to be best.
            // Therefore with 15 valves, we want roughly half of those 15 bits to
            // be each binary value. Check there's between 6 and 9 ones.
            // Not only does this reduce the number of partitions, it also makes
            // testing them much faster, as neither set of the partition has more
            // than 9 valves to visit.
            if (part_idx as i32).count_ones() < 6 || (part_idx as i32).count_ones() > 9 {
                continue;
            }
            let mut partition = HashSet::new();
            for (idx, valve) in valves.iter().enumerate() {
                if (part_idx & (1 << idx)) != 0 {
                    partition.insert(valve.clone());
                }
            }
            result.push(partition);
        }

        result
    }

    fn calculate_joint(&self, remain: i32) -> i32 {
        let init_valve_state: HashMap<String, ValveState> = self
            .valves
            .iter()
            .map(|v| (v.0.clone(), ValveState::Closed))
            .collect();

        let mut best_pressure = 0;

        let all_partitions = self.all_partitions();
        //println!("Testing {} partitions", all_partitions.len());

        for (_idx, partition) in all_partitions.iter().enumerate() {
            let mut my_valve_state = init_valve_state.clone();
            let mut elephant_valve_state = init_valve_state.clone();

            for valve_name in init_valve_state.keys() {
                if partition.contains(valve_name) {
                    my_valve_state.insert(valve_name.clone(), ValveState::Opened);
                    elephant_valve_state.insert(valve_name.clone(), ValveState::Closed);
                } else {
                    my_valve_state.insert(valve_name.clone(), ValveState::Closed);
                    elephant_valve_state.insert(valve_name.clone(), ValveState::Opened);
                }
            }

            my_valve_state.insert("AA".to_string(), ValveState::Opened);
            elephant_valve_state.insert("AA".to_string(), ValveState::Opened);

            let partition_result = self.brute_force(remain, "AA".to_string(), my_valve_state)
                + self.brute_force(remain, "AA".to_string(), elephant_valve_state);
            if partition_result > best_pressure {
                //println!("Partition# {}; best so far: {}", _idx, partition_result);
                best_pressure = partition_result;
            }
        }

        best_pressure
    }
}

pub fn step1() {
    let mut volcano = Volcano::new("inputs/day16.txt");
    volcano.reduce();
    //volcano.render_dot();
    volcano.calc_all_pairs_shortest();
    println!("Best pressure in 30 minutes: {}", volcano.calculate(30));
}

pub fn step2() {
    let mut volcano = Volcano::new("inputs/day16.txt");
    volcano.reduce();
    volcano.calc_all_pairs_shortest();
    println!("With two valve openers in 26 minutes: {}", volcano.calculate_joint(26));
}
