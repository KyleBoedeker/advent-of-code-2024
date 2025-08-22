use std::io::{self, BufRead};

use regex::Regex;

struct Stone {
    value: u64,
    count: u64,
    new_count: u64,
    transition_values: (u64, Option<u64>),
}

impl Stone {
    fn new(value: u64) -> Self {
        Stone {
            value,
            count: 1,
            new_count: 0,
            transition_values: Stone::get_transition_values(value),
        }
    }

    fn get_transition_values(value: u64) -> (u64, Option<u64>) {
        // rule 1: replace 0 with 1
        if value == 0 {
            return (1, None);
        }
        // rule 2: split stones in half with even # of digits
        let stringy_stone = value.to_string();
        if stringy_stone.len() % 2 == 0 {
            let (left, right) = stringy_stone.split_at(stringy_stone.len() / 2);
            return (left.parse().unwrap(), Some(right.parse().unwrap()));
        }
        // rule 3: multiply by 2024
        (value * 2024, None)
    }
}

struct StoneVec {
    stones: Vec<Stone>,
}

impl StoneVec {
    fn new() -> Self {
        StoneVec { stones: Vec::new() }
    }

    fn from(stone_values: Vec<u64>) -> Self {
        let mut sv = StoneVec::new();
        for v in stone_values {
            sv.stones.push(Stone::new(v));
        }
        sv
    }

    fn transition(&mut self) {
        // value, count
        let mut value_counts: Vec<(u64, u64)> = vec![];
        for st in self.stones.iter() {
            value_counts.push((st.transition_values.0, st.count));
            if let Some(v1) = st.transition_values.1 {
                value_counts.push((v1, st.count));
            }
        }
        // add the missing stones
        for (val, _count) in value_counts.iter() {
            if !self.stones.iter().any(|s| s.value == *val) {
                self.stones.push(Stone::new(*val));
            }
        }
        // update new counts
        for (val, count) in value_counts {
            let st: &mut Stone = self.stones.iter_mut().find(|s| s.value == val).unwrap();
            st.new_count += count;
        }
        // swap count out and reset new counts
        for st in self.stones.iter_mut() {
            st.count = st.new_count;
            st.new_count = 0;
        }
    }

    fn total_sum(&self) -> u64 {
        self.stones.iter().map(|v| v.count).sum()
    }
}

fn main() {
    // read stdin
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer).unwrap();
    }

    // Parse stdin as a list of numbers
    let re = Regex::new(r"(\d+)").unwrap();
    let stones: Vec<u64> = re
        .captures_iter(&buffer)
        .map(|v| v.get(0).unwrap().as_str().parse().unwrap())
        .collect();

    let mut stone_vec = StoneVec::from(stones);

    let num_blinks = 75;
    for _ in 0..num_blinks {
        stone_vec.transition();
    }

    println!(
        "# of stones after blinking {} times: {}",
        num_blinks,
        stone_vec.total_sum()
    );
}
