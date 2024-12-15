use regex::Regex;
use std::io;

fn main() {
    // match either:
    //   mul(x,y)
    //     (where x and y could be 1-3 digits)
    //   do()
    //   don't()
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut total: u32 = 0;
    let mut instruction_enabled = true;

    let stdin = io::read_to_string(io::stdin()).unwrap();
    for m in re.captures_iter(&stdin) {
        match m[0].as_ref() {
            "don't()" => instruction_enabled = false,
            "do()" => instruction_enabled = true,
            _ => {
                if instruction_enabled {
                    total += m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap()
                }
            }
        }
    }
    println!("total is: {}", total);
}
