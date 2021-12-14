use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // read input from file
    let file = File::open("src/input").expect("file not found");
    let reader = BufReader::new(file);

    let mut depth = 0;
    let mut h_pos = 0;
    let mut aim = 0;

    for line in reader.lines() {

        // read line as string
        let line = line.unwrap();

        let mut splice = line.split_whitespace();

        // read direction
        let direction = splice.next().unwrap();

        // read distance
        let distance: i32 = splice.next().unwrap().parse().unwrap();

        // move
        match direction {
            "forward" => {h_pos += distance; depth += aim*distance;},
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => panic!("invalid direction"),
        }

    }

    println!("{}", h_pos * depth);
    Ok(())
}