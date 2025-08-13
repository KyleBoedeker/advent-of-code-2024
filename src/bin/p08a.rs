use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn antinode(&self, other: &Point) -> Point {
        // return the antinode of a given point relative to another
        return Point { row: other.row - self.row + other.row, col: other.col - self.col + other.col};
    }

    fn bounded_by(&self, max_size: i32) -> bool {
        return self.col >= 0 && self.row >= 0 && self.col < max_size && self.row < max_size;
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
            let positions = antenna_positions.entry(val).or_insert(vec![]);
            positions.push(Point { row: row as i32, col: col as i32});
        }
        // update row_counter
        puzzle_size += 1;
    }

    let mut antinode_positions: HashSet<Point> = HashSet::new();
    for positions in antenna_positions.values() {
        for p1 in positions.iter() {
            for p2 in positions.iter() {
                let antinode = p1.antinode(p2);
                if !antinode.bounded_by(puzzle_size) || positions.contains(&antinode) {
                    continue
                }
                antinode_positions.insert(p1.antinode(p2));
            }
        }
    }

    println!("# of unique antinode positions = {}", antinode_positions.len());

    // Debug to show puzzle with antinodes projected
    for row in 0..puzzle_size {
        for col in 0..puzzle_size {
            let p = Point { row, col };

            let mut c = None;
            for (ac, positions) in antenna_positions.iter() {
                if positions.contains(&p) {
                    c = Some(*ac);
                    break
                }
            }
            if c.is_none() && antinode_positions.contains(&p) {
                c = Some('#');
            }
            print!("{}", c.unwrap_or('.'));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antenna_bounding() {
        let p = Point { row: 3, col: 6 };
        assert!(!p.bounded_by(6));
        assert!(p.bounded_by(7));

        let p = Point { row: -1, col: 0 };
        assert!(!p.bounded_by(1));
        assert!(!p.bounded_by(10));
    }

    #[test]
    fn antinode() {
        /* 'x' = antinode, '-' = antenna (tx)
         *     0 1 2 3 4 5 6 7 8
         *   0 x
         *   1     -
         *   2         -
         *   3             x
         */
        let tx0 = Point { row: 1, col: 2 };
        let tx1 = Point { row: 2, col: 4 };

        // compare tx1 and tx2
        assert_eq!(tx0.antinode(&tx1), Point { row: 3, col: 6 });
        assert_eq!(tx1.antinode(&tx0), Point { row: 0, col: 0 });

        /* 'x' = antinode, '-' = antenna (tx)
         *     0
         *   0 -
         *   1 -
         *   2 x
         */
        let tx0 = Point { row: 0, col: 0 };
        let tx1 = Point { row: 1, col: 0 };

        assert_eq!(tx1.antinode(&tx0), Point { row: -1, col: 0 });
        assert_eq!(tx0.antinode(&tx1), Point { row: 2, col: 0 });

    }
}
