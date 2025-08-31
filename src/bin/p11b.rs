use std::{
    collections::HashMap,
    io::{self, BufRead},
    mem,
    time::Instant,
};

use regex::Regex;

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

fn main() {
    let start_instant = Instant::now();
    // read stdin
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer).unwrap();
    }

    // Parse stdin as a list of numbers
    let re = Regex::new(r"(\d+)").unwrap();
    let input_stones: Vec<u64> = re
        .captures_iter(&buffer)
        .map(|v| v.get(0).unwrap().as_str().parse().unwrap())
        .collect();

    // next available stone index (used to index other arrays)
    let mut next_stone_idx: usize = 0;
    // lookup a stone's index by value (used to index other arrays)
    let mut stone_idx_lookup: HashMap<u64, usize> = HashMap::default();
    // Using fixed-size arrays since there _shouldn't_ be more than ~4k stones from my experiments
    // stone counts is the pre-start-of-transition counts, stone_counts_new is used during the transition
    let mut stone_counts = [0u64; 4096];
    let mut stone_counts_new = [0u64; 4096];

    // Add the initial set of stones and the count (1 by default) for each stone in the input
    for s in input_stones {
        let idx = stone_idx_lookup.entry(s).or_insert(next_stone_idx);
        if *idx == next_stone_idx {
            next_stone_idx += 1;
        }
        stone_counts[*idx] += 1;
    }

    let num_blinks = 75;
    for _ in 0..num_blinks {
        // get a snapshot of the stones we need to transition
        let stones: Vec<u64> = stone_idx_lookup.keys().cloned().collect();

        for stone_value in stones {
            // could probably move this count outside the loop
            let count = stone_counts[*stone_idx_lookup.get(&stone_value).unwrap()];

            let (t1, t2) = get_transition_values(stone_value);

            // get the index of the new stone and add the count of the transitioning stone
            let idx = stone_idx_lookup.entry(t1).or_insert(next_stone_idx);
            if *idx == next_stone_idx {
                next_stone_idx += 1;
            }
            stone_counts_new[*idx] += count;

            // ditto for t2
            if let Some(t2) = t2 {
                let idx = stone_idx_lookup.entry(t2).or_insert(next_stone_idx);
                if *idx == next_stone_idx {
                    next_stone_idx += 1;
                }
                stone_counts_new[*idx] += count;
            }
        }

        // In with the new, out with the old
        mem::swap(&mut stone_counts, &mut stone_counts_new);
        stone_counts_new.iter_mut().for_each(|v| *v = 0);
    }

    println!(
        "# of stones after blinking {} times is {}. Took {:.3} seconds.",
        num_blinks,
        stone_counts.iter().sum::<u64>(),
        start_instant.elapsed().as_secs_f32(),
    );
}
