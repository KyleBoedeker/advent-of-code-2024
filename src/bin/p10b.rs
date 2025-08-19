use std::io::{self, BufRead};

type Map = Vec<Vec<u32>>;
// row then column
type Position = (i32, i32);

const DIRECTIONS: [Position; 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

// recursive (since it'll only recur to depth 10 and this is the hackiest thing I could think of
// doing... ;)
fn traverse(map: &Map, pos: Position, prev_val: u32, count: &mut usize) {
    for (r, c) in DIRECTIONS.iter() {
        // our new positions as prime positions
        let rp = r + pos.0;
        let cp = c + pos.1;
        // UGLY: rely on out-of-bounds for negatives when we cast i32 -> usize
        if let Some(val) = map.get(rp as usize).and_then(|row| row.get(cp as usize)) {
            // need to ensure we check the previous value so we don't consider '0' -> '9' a path
            if *val == 9 && prev_val == 8 {
                *count += 1;
            } else if prev_val + 1 == *val {
                // Keeps liftin gme higher and higher...
                traverse(map, (rp, cp), *val, count);
            }
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines();

    // Parse the map as a 2d matrix (vec of vecs)
    let map: Map = lines
        .map(|v| {
            v.unwrap()
                .chars()
                .map(|v| v.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let sum_of_trailhead_scores: usize = map
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_col_idx, val)| **val == 0)
                .map(|(col_idx, _val)| {
                    // Phew, we found a '0' in our map, let's try traversing to all peaks
                    let mut count = 0;
                    traverse(&map, (row_idx as i32, col_idx as i32), 0, &mut count);
                    count
                })
                .sum::<usize>()
        })
        .sum();

    println!("Sum of trailhead scores: {}", sum_of_trailhead_scores);
}
