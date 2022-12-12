struct Monkey {
    pub items: Vec<i64>,
    op: char,
    immediate: i64,
    test_divisor: i64,
    target_true: usize,
    target_false: usize,
    inspected: i64,
}

impl Monkey {
    pub fn new(
        items: &[i64],
        op: char,
        immediate: i64,
        test_divisor: i64,
        target_true: usize,
        target_false: usize,
    ) -> Self {
        Self {
            items: items.to_vec(),
            op,
            immediate,
            test_divisor,
            target_false,
            target_true,
            inspected: 0,
        }
    }

    pub fn turn(&mut self, divide: bool) -> Vec<(i64, usize)> {
        let base = if divide {
            // For consistency, to avoid not doing the mod operation
            i64::MAX
        } else {
            // The monkey test_divisors are the following - all primes, so a
            // common modulo base of the product of these will be equal
            3 * 13 * 2 * 11 * 5 * 17 * 19 * 7
        };

        let mut result = vec![];
        for item in &self.items {
            // inspect item
            let mut value = match self.op {
                '*' => (item * self.immediate) % base,
                '+' => (item + self.immediate) % base,
                's' => (item * item) % base,
                _ => {
                    panic!("Unknown operation");
                }
            };
            self.inspected += 1;

            if divide {
                value /= 3;
            }

            let item_target = if value % self.test_divisor == 0 {
                self.target_true
            } else {
                self.target_false
            };
            result.push((value, item_target));
        }
        self.items.clear();
        result
    }
}

fn generate_data() -> Vec<Monkey> {
    // I couldn't be bothered to write a parser, though I probably should have as
    // I made a mistake transcribing the data...
    let m0 = Monkey::new(&[64, 89, 65, 95], '*', 7, 3, 4, 1);
    let m1 = Monkey::new(&[76, 66, 74, 87, 70, 56, 51, 66], '+', 5, 13, 7, 3);
    let m2 = Monkey::new(&[91, 60, 63], 's', 0, 2, 6, 5);
    let m3 = Monkey::new(&[92, 61, 79, 97, 79], '+', 6, 11, 2, 6);
    let m4 = Monkey::new(&[93, 54], '*', 11, 5, 1, 7);
    let m5 = Monkey::new(&[60, 79, 92, 69, 88, 82, 70], '+', 8, 17, 4, 0);
    let m6 = Monkey::new(&[64, 57, 73, 89, 55, 53], '+', 1, 19, 0, 5);
    let m7 = Monkey::new(&[62], '+', 4, 7, 3, 2);

    vec![m0, m1, m2, m3, m4, m5, m6, m7]
}

pub fn step1() {
    let mut monkeys = generate_data();

    for _ in 0..20 {
        // RUST: is there a nicer way of modifying multiple entries of
        // a container than just indexing (rather than iterating over)
        // the container?
        for m in 0..monkeys.len() {
            let thrown = monkeys[m].turn(true);
            for (v, t) in thrown {
                monkeys[t].items.push(v);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    println!("{}", monkeys[0].inspected * monkeys[1].inspected);
}

pub fn step2() {
    let mut monkeys = generate_data();

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            let thrown = monkeys[m].turn(false);
            for (v, t) in thrown {
                monkeys[t].items.push(v);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    println!("{}", monkeys[0].inspected * monkeys[1].inspected);
}
