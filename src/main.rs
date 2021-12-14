use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()> {
    // read input from file
    let file = File::open("src/input").expect("file not found");
    let reader = BufReader::new(file);

    // create array of three integers to store the input
    let mut input: [i32; 3] = [0; 3];
    // previous count of the three integers
    let mut prev_count = 0;
    let mut input_counter = 0;
    let mut increase_counter: i32 = 0;
    let mut row = 0;

    for line in reader.lines() {
        let value: i32 = line.unwrap().parse().unwrap();
        input[input_counter] = value;
        input_counter += 1;
        input_counter %= 3;
        row += 1;

        if row >= 3 {
            analyze_slice(&input, &mut prev_count, &mut increase_counter)
        }
    }

    println!("{}", increase_counter);
    Ok(())
}

fn analyze_slice(slice: &[i32; 3], prev_count: &mut i32, counter: &mut i32) {
    let sum = slice.iter().sum::<i32>();
    if *prev_count != 0 && sum > *prev_count {
        *counter += 1;
    }
    *prev_count = sum;
}
