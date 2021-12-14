use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // read input from file
    let file = File::open("src/input").expect("file not found");
    let reader = BufReader::new(file);

    let mut input_rows = 0;

    // create an array of 12 integers
    let mut input_values: [i32; 12] = [0; 12];

    for line in reader.lines() {
        // read line as string
        let line = line.unwrap();
        // iterate over each character in the line
        for (i, c) in line.chars().enumerate() {
            // convert character to integer
            if c == '1' {
                input_values[i] += 1;
            }
        }

        input_rows += 1;
    }

    input_values.reverse();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let half_value = input_rows / 2;

    let mut index = 0;
    for value in input_values {
        let magic_number = i32::pow(2, index);

        if value > half_value {
            gamma_rate += magic_number;
        } else {
            epsilon_rate += magic_number;
        }

        println!("{}", magic_number);
        index += 1;
    }

    println!("{}", gamma_rate);
    println!("{}", epsilon_rate);
    println!("{}", gamma_rate * epsilon_rate);

    Ok(())
}
