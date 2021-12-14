use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // read input from file
    let file = File::open("src/input").expect("file not found");
    let reader = BufReader::new(file);

    let mut input: Vec<i32> = Vec::new();
    // previous count of the three integers
    let mut increase_counter: i32 = 0;

    for line in reader.lines() {
        let value: i32 = line.unwrap().parse().unwrap();
        input.push(value);

        if input.len() < 4 {
            continue;
        }

        if analyze_slice(&input) {
            increase_counter += 1;
        }

        input.remove(0);
    }

    println!("{}", increase_counter);
    Ok(())
}

fn analyze_slice(slice: &[i32]) -> bool {
    let prev_count = slice[0..3].iter().sum::<i32>();
    let current_count = slice[1..4].iter().sum::<i32>();

    current_count > prev_count
}
