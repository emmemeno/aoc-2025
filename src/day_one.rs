#![allow(dead_code)]

use std::{fs::File, io::Read, io::{Write, BufWriter}};

pub fn day_one() {
    let output_file = File::create("asset/output").unwrap();
    let mut output_writer = BufWriter::new(&output_file);
    let mut input_file = File::open("asset/input").unwrap();
    let mut input_data = String::new();
    input_file.read_to_string(&mut input_data).unwrap();
    let lines: Vec<&str> = input_data.split_terminator("\n").collect();
    let mut number: isize = 50;
    let mut zero_occurrences = 0;

    let _ = writeln!(output_writer, "Starting position: {number}");
    for line in lines.iter() {
        let old_number = number;
        //split the line
        let (dir, value) = line.split_at(1);
        // convert str to usize
        let mut value_n = value.parse::<isize>().unwrap();
        // if there is no rest, simply add zero occurrencies by division
        if value_n % 100 == 0 {
            zero_occurrences += value_n / 100;
            continue;
        }
        // if value is more than 100, add one occurrency and calculate the rest
        if value_n > 100 {
            zero_occurrences += value_n / 100;
            value_n = value_n % 100;
        }
        
        match dir {
            "R" => {
                number += value_n;
                if number == 100 {
                    zero_occurrences += 1;
                    number = 0;
                }
                if number > 100 {
                    zero_occurrences += 1;
                    number = number % 100;
                }
            },
            "L" => {
                number -= value_n;
                if number == 0 {
                    zero_occurrences += 1;
                }
                if number < 0 {
                    // doesnt count 2 times
                    if old_number !=0 {
                        zero_occurrences += 1;
                    }
                    number = 100 + number;
                }
            },
            //wrong direction, should not be present in input
            _ => panic!(),
        };
        let _ = writeln!(output_writer, "Moving {old_number} to {dir} by {value} ({value_n}) -> {number} |{zero_occurrences}|");
    }
    println!("{}", zero_occurrences);

}
