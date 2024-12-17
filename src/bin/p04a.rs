use std::io::{self, BufRead};

// each direction letters could be in
// const directions: [[i32; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];

// return # of valid "XMAS" words detected starting at the x-y index provided
fn check(
    puz: &Vec<String>,
    x: usize,
    y: usize,
    xmas_idx: usize,
    xdir: &Option<bool>,
    ydir: &Option<bool>,
) -> bool {
    let c = puz[y].chars().nth(x).unwrap();
    if c != "XMAS".chars().nth(xmas_idx).unwrap() {
        return false;
    }
    if c == 'S' {
        return true;
    }
    if (x == 0 && *xdir == Some(false)) || (x == puz.len() - 1 && *xdir == Some(true)) {
        return false;
    }
    if (y == 0 && *ydir == Some(false)) || (y == puz.len() - 1 && *ydir == Some(true)) {
        return false;
    }
    let xnew = match *xdir {
        Some(true) => x + 1,
        Some(false) => x - 1,
        None => x,
    };
    let ynew = match *ydir {
        Some(true) => y + 1,
        Some(false) => y - 1,
        None => y,
    };
    return check(puz, xnew, ynew, xmas_idx + 1, xdir, ydir);
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut xmas_count = 0;

    let directions: Vec<(Option<bool>, Option<bool>)> = vec![
        (Some(false), Some(false)),
        (Some(false), None),
        (Some(false), Some(true)),
        (None, Some(false)),
        (None, Some(true)),
        (Some(true), Some(false)),
        (Some(true), None),
        (Some(true), Some(true)),
    ];
    for y in 0..lines.len() {
        for x in 0..lines.len() {
            xmas_count += directions
                .iter()
                .map(|(xdir, ydir)| check(&lines, x, y, 0, xdir, ydir) as u32)
                .sum::<u32>()
        }
    }

    println!("# of \"XMAS\" in input = {}", xmas_count);
}
