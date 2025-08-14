use std::io::{self, BufRead};
use std::collections::VecDeque;

#[derive(Debug)]
struct FileBlock {
    id: usize,
    isfree: bool,
    start: usize,
    size: usize,
}

fn main() {
    let line: String = io::stdin().lock().lines().next().unwrap().unwrap();


    let mut start = 0;
    let mut deq = VecDeque::new();

    for (idx, c) in line.chars().enumerate() {
        let isfree: bool = (idx % 2) == 1;
        let size = c.to_digit(10).unwrap() as usize;
        let id = (idx + 1) / 2;
        deq.push_back(FileBlock { id, isfree, start, size } );
        start += size;
    }

    // compare first element to last element
    // if front can fit back, then sub from back and remove the back
    // elif the back is too big, then pop the front
    let mut id_crc = 0;
    loop {
        let mut front = deq.front_mut().unwrap();

        if !front.isfree {
            id_crc += front.id * front.start;
        }
        if deq.len() <= 1 {
            break
        }
        if !front.isfree {
            continue;
        }
        // more elements!
        let mut back = deq.back_mut().unwrap();
        if 

        if  {

            
        }
        break
    }

    dbg!(&deq);
    println!("{}", deq.len());
}
