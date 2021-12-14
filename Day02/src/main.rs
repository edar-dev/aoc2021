use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // read input from file
    let file = File::open("src/input").expect("file not found");
    let reader = BufReader::new(file);

    let mut v_pos = 0;
    let mut h_pos = 0;

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
            "forward" => h_pos += distance,
            "up" => v_pos -= distance,
            "down" => v_pos += distance,
            _ => panic!("invalid direction"),
        }

    }

    println!("{}", v_pos * h_pos);
    Ok(())
}