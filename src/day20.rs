use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct Node {
    value: i64,
    next_pos: usize,
    prev_pos: usize,
}

fn decode(ciphertext: &[i64], mix_count: i32) -> Vec<i64> {
    // Array-based circular linked list
    let mut ll: Vec<Node> = vec![];
    for element in 0..ciphertext.len() {
        ll.push(Node {
            value: ciphertext[element],
            next_pos: (element + 1) % ciphertext.len(),
            prev_pos: (element + ciphertext.len() - 1) % ciphertext.len(),
        });
    }

    for _ in 0..mix_count {
        // For each element in ciphertext, remove (joining the prev/next of this
        // nodes next/prev) and reinsert at the required target position.
        for node_idx in 0..ciphertext.len() {
            // -1 in mod operation as we don't count the moving element
            let incr = ciphertext[node_idx] % (ciphertext.len() as i64 - 1);
            if incr == 0 {
                continue;
            }

            // get position in our list of nodes
            let node_idx_next = ll[node_idx].next_pos;
            let node_idx_prev = ll[node_idx].prev_pos;

            // Extract node from ll, splicing the two surrounding nodes
            ll[node_idx_next].prev_pos = node_idx_prev;
            ll[node_idx_prev].next_pos = node_idx_next;

            // Determine new position
            let mut node_idx_new = node_idx;
            let node_idx_new_next;
            let node_idx_new_prev;
            if incr > 0 {
                for _ in 0..incr {
                    node_idx_new = ll[node_idx_new].next_pos;
                }
                node_idx_new_next = ll[node_idx_new].next_pos;
                node_idx_new_prev = node_idx_new;
            } else {
                for _ in 0..incr.abs() {
                    node_idx_new = ll[node_idx_new].prev_pos;
                }
                node_idx_new_next = node_idx_new;
                node_idx_new_prev = ll[node_idx_new].prev_pos;
            }

            // Splice us back in to the target location
            ll[node_idx_new_prev].next_pos = node_idx;
            ll[node_idx_new_next].prev_pos = node_idx;
            ll[node_idx].next_pos = node_idx_new_next;
            ll[node_idx].prev_pos = node_idx_new_prev;
        }
    }

    let mut result = vec![];
    let mut pos = 0;
    while result.len() < ll.len() {
        result.push(ll[pos].value);
        pos = ll[pos].next_pos;
    }

    result
}

pub fn step1() {
    let ciphertext: Vec<i64> = read_list("inputs/day20.txt")
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    // Numbers are in approx range -10000..10000, and have repeats.
    let cleartext = decode(&ciphertext, 1);

    let zero_idx = cleartext.iter().position(|&x| x == 0).unwrap();
    let zero_1000 = (zero_idx + 1000) % cleartext.len();
    let zero_2000 = (zero_idx + 2000) % cleartext.len();
    let zero_3000 = (zero_idx + 3000) % cleartext.len();

    println!(
        "Sum of 1000th, 2000th & 3000th after zero: {}",
        cleartext[zero_1000] + cleartext[zero_2000] + cleartext[zero_3000]
    );
}

pub fn step2() {
    let ciphertext: Vec<i64> = read_list("inputs/day20.txt")
        .iter()
        .map(|x| x.parse::<i64>().unwrap() * 811589153i64)
        .collect();

    let cleartext = decode(&ciphertext, 10);

    let zero_idx = cleartext.iter().position(|&x| x == 0).unwrap();
    let zero_1000 = (zero_idx + 1000) % cleartext.len();
    let zero_2000 = (zero_idx + 2000) % cleartext.len();
    let zero_3000 = (zero_idx + 3000) % cleartext.len();

    println!(
        "Sum of 1000th, 2000th & 3000th after zero: {}",
        cleartext[zero_1000] + cleartext[zero_2000] + cleartext[zero_3000]
    );
}
