mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;

use std::{fs::File, io::Read};
use std::time::Instant;

enum InputMode {
    Example,
    Normal,
}

fn load_input(file_name: &str) -> String {
    let mut input_file = File::open(file_name).unwrap();
    let mut data = String::new();
    input_file.read_to_string(&mut data).unwrap();
    data
}

fn main() {

    let now = Instant::now();

    day_nine::part_two();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

