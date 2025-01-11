use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut lines = io::stdin().lock().lines().enumerate();

    let mut map = vec![];
    let mut player_position: Option<(usize, usize)> = None;

    while let Some((row_idx, Ok(line))) = lines.next() {
        let mut row = vec![];
        for (col_idx, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => -1, // where been
                '.' => 0,
                '*' => 1,
                '^' => {
                    player_position = Some((row_idx, col_idx));
                    1
                }
                v => panic!("Weird character spotted in the map '{}'", v),
            })
        }
        map.push(row);
    }

    let (mut player_row, mut player_col) = player_position.expect("Player not found!");

    let mut dirs = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ]
    .iter()
    .cycle();
    let mut dir = dirs.next().unwrap();

    loop {
        let (new_row, new_col) = match dir {
            Direction::Up => (player_row - 1, player_col),
            Direction::Down => (player_row + 1, player_col),
            Direction::Left => (player_row, player_col - 1),
            Direction::Right => (player_row, player_col + 1),
        };
        // rely on rust's inbuilt bounds checking - underflow from zero is safe since
        // we'll wrap to some index beyond the map boundary
        if let Some(map_row) = map.get_mut(new_row) {
            if let Some(tile) = map_row.get_mut(new_col) {
                // if the new tile is an obstacle, change directions
                if *tile == -1 {
                    dir = dirs.next().unwrap();
                    continue;
                }
                // mark we've been here and update player position
                *tile = 1;
                (player_row, player_col) = (new_row, new_col);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let total: i32 = map
        .iter()
        .map(|row| row.iter().filter(|&v| *v > 0).sum::<i32>())
        .sum();

    println!("# of spaces traveled is: {}", total);
}
