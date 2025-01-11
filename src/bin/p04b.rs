use std::io::{self, BufRead};

fn check(puz: &Vec<String>, x: usize, y: usize) -> bool {
    let c = puz[y].chars().nth(x).unwrap();

    // check that center is "A"
    if c != 'A' {
        return false;
    }

    // bounds checks
    if x == 0 || x == puz.len() - 1 || y == 0 || y == puz.len() - 1 {
        return false;
    }

    // upper left, upper right, etc.
    let ul = puz[y - 1].chars().nth(x - 1).unwrap();
    let ur = puz[y - 1].chars().nth(x + 1).unwrap();
    let ll = puz[y + 1].chars().nth(x - 1).unwrap();
    let lr = puz[y + 1].chars().nth(x + 1).unwrap();

    // upper left, lower right make "MAS" or "SAM"
    if ul == 'M' && lr == 'S' || ul == 'S' && lr == 'M' {
        // upper right, lower left make "MAS" or "SAM"
        if ur == 'M' && ll == 'S' || ur == 'S' && ll == 'M' {
            return true;
        }
    }
    false
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut xmas_count = 0;

    for y in 0..lines.len() {
        for x in 0..lines.len() {
            xmas_count += check(&lines, x, y) as u32;
        }
    }

    println!("# of \"X-MAS\"-es  in input = {}", xmas_count);
}
