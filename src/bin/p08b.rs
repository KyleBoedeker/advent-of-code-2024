use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}


impl Point {
    fn antinodes(&self, other: &Point, max_size: i32) -> Vec<Point> {
        // return the antinodes of a given point relative to another (project self through other)
        let mut n = other.clone();
        let (delta_row, delta_col) = (other.row - self.row, other.col - self.col);
        let mut anodes = vec![];
        loop {
            n.row += delta_row;
            n.col += delta_col;
            if !n.bounded_by(max_size) {
                break
            }
            anodes.push(n.clone());
        }
        return anodes;
    }

    fn bounded_by(&self, max_size: i32) -> bool {
        self.col >= 0 && self.row >= 0 && self.col < max_size && self.row < max_size
    }
}

fn main() {
    let mut antenna_positions: HashMap<char, Vec<Point>> = HashMap::new();

    let mut puzzle_size = 0;
    let lines = io::stdin().lock().lines();
    for (row, line) in lines.enumerate() {
        for (col, val) in line.unwrap().chars().enumerate() {
            if val == '.' {
                continue;
            }
            let positions = antenna_positions.entry(val).or_default();
            positions.push(Point {
                row: row as i32,
                col: col as i32,
            });
        }
        puzzle_size += 1;
    }

    let mut antinode_positions: HashSet<Point> = HashSet::new();
    for positions in antenna_positions.values() {
        antinode_positions.extend(positions.iter().map(|v| v.clone()));
        for p1 in positions.iter() {
            for p2 in positions.iter() {
                if p1 == p2 {
                    // skip identical points (got lucky on part 1 here)
                    continue
                }
                antinode_positions.extend(p1.antinodes(p2, puzzle_size));
            }
        }
    }

    println!(
        "# of unique antinode positions = {}",
        antinode_positions.len()
    );
}
