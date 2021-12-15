use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // read input from file
    let file_content = read_file_content("src/input");

    let oxygen_generator_rating = find_value(&file_content, 0, true);

    let co2_scrubber_rating = find_value(&file_content, 0,false);

    println!(
        "{} x {} = {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );

    Ok(())
}

fn find_value(input_list: &Vec<Vec<u32>>, pos: usize, get_bigger: bool) -> u32 {
    if input_list.len() == 1 {
        from_binary_string_to_decimal(&input_list[0].clone())
    } else {
        let most_common_char: u32 = get_next_bit_value(input_list, pos, get_bigger);
        let new_input_list: Vec<Vec<u32>> = input_list
            .into_iter()
            .filter(|&input| input[pos] == most_common_char)
            .cloned()
            .collect();
            find_value(&new_input_list, pos + 1, get_bigger)
    }
}

fn get_next_bit_value(input_list: &Vec<Vec<u32>>, pos: usize, get_bigger: bool) -> u32 {
    let mut char_counts: Vec<u32> = vec![0; 2];
    for input in input_list {
        char_counts[input[pos] as usize] += 1;
    }
    if get_bigger {
        if char_counts[0] > char_counts[1] {
            0
        } else {
            1
        }
    } else {
        if char_counts[0] <= char_counts[1] {
            0
        } else {
            1
        }
    }
}

fn read_file_content(file_path: &str) -> Vec<Vec<u32>> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);

    let mut input_rows: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        // read line as string
        let line = line.unwrap();
        // iterate over each character in the line
        let mut input_row: Vec<u32> = Vec::new();
        for character in line.chars() {
            input_row.push(character.to_digit(10).unwrap());
        }
        input_rows.push(input_row);
    }

    input_rows
}

fn from_binary_string_to_decimal(input: &Vec<u32>) -> u32 {
    let mut result = 0;
    let mut multiplier = 1;

    let mut inverted_binary = input.clone();
    inverted_binary.reverse();
    for c in inverted_binary {
        print!("{:?}", c);
        result += multiplier * c;
        multiplier *= 2;
    }
    println!("");

    result
}
