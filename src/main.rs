mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;

use std::{fs::File, io::Read};
use std::time::Instant;
fn load_input(file_name: &str) -> String {
    let mut input_file = File::open(file_name).unwrap();
    let mut data = String::new();
    input_file.read_to_string(&mut data).unwrap();
    data
}

fn main() {

    let now = Instant::now();

    // day_one::day_one()
    // day_two::day_two_part_one()
    // day_two::part_two();
    // day_three::part_one();
    // day_three::part_two();
    // day_four::part_one();
    // day_four::part_two();
    // day_five::part_one();
    // day_five::part_two();
    // day_six::part_one();
    // day_six::part_two();
    day_seven::part_two(&load_input("input/input-day7"));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

