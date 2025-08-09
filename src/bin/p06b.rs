use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    col: i32,
    row: i32,
    dir: Direction,
}

impl Position {
    fn update_dir(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Right => self.dir = Direction::Down,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
        }
    }
}

fn guard_infinitely_loops(map: &[Vec<i32>], mut player_position: Position) -> bool {
    let mut visited_positions = vec![];

    for _infinite_loop_guard in 0..1_000_000 {
        let (new_row, new_col) = match player_position.dir {
            Direction::Up => (player_position.row - 1, player_position.col),
            Direction::Down => (player_position.row + 1, player_position.col),
            Direction::Left => (player_position.row, player_position.col - 1),
            Direction::Right => (player_position.row, player_position.col + 1),
        };

        // guard does not infinitely loop (escaped the puzzle)
        if new_row < 0
            || new_row >= map.len() as i32
            || new_col < 0
            || new_col >= map[0].len() as i32
        {
            return false;
        }

        let tile = map
            .get(new_row as usize)
            .unwrap()
            .get(new_col as usize)
            .unwrap();
        // if the new tile is an obstacle (negative values) then change directions
        if *tile < 0 {
            if visited_positions.contains(&player_position) {
                // found a loop! We've been here before
                // println!("loop found!");
                return true;
            }
            visited_positions.push(player_position.clone());
            // don't forget to update the player's direction
            player_position.update_dir();
        } else {
            player_position.row = new_row;
            player_position.col = new_col;
        }
    }
    panic!("Infinite loop detected");
}

fn main() {
    let mut lines = io::stdin().lock().lines().enumerate();

    let mut map = vec![];
    let mut player_row_col: Option<(usize, usize)> = None;

    while let Some((row_idx, Ok(line))) = lines.next() {
        let mut row = vec![];
        for (col_idx, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => -1, // obstruction
                '.' => 0,  // empty space
                '^' => {
                    // player
                    player_row_col = Some((row_idx, col_idx));
                    // special value for this to enable both tile != 0 and tile < 0 comparisons
                    1
                }
                v => panic!("Weird character spotted in the map '{v}'"),
            })
        }
        map.push(row);
    }

    let (player_row, player_col) = player_row_col.expect("Player not found!");
    let player_position = Position {
        col: player_col as i32,
        row: player_row as i32,
        // player is always facing up to start with
        dir: Direction::Up,
    };

    let mut num_valid_blocker_positions = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            // check if the position is already an obstacle or the guard's starting position
            if map[row][col] != 0 {
                continue;
            }
            map[row][col] = -1;
            if guard_infinitely_loops(&map, player_position.clone()) {
                // println!("(row, col) = ({row}, {col})");
                num_valid_blocker_positions += 1;
            }
            map[row][col] = 0;
        }
    }

    println!(
        "# of positions where block can be placed: {num_valid_blocker_positions}"
    );
}
