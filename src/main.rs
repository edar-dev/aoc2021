use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()> {

    // read input from file
    let file = File::open("src/input").expect("file not found");
    let reader = BufReader::new(file);

    let mut row = 0;
    let mut prev_value = 0;
    let mut counter =0;
    for line in reader.lines() {
        if row == 0 {
            prev_value = line.unwrap().parse::<i32>().unwrap();
            println!("{} (N/A - no previous measurement)" , prev_value);
        } else {
            let current_value = line.unwrap().parse::<i32>().unwrap();
            if current_value > prev_value {
                println!("{} (increased)", current_value);
                counter += 1;
            } else {
                println!("{} (decreased)" , current_value);
            }
            prev_value = current_value;
        }
        row += 1;
    }
    println!("{} FIANL VALUE", counter);

    Ok(())

}
