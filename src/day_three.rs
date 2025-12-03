#![allow(dead_code)]
use std::cmp::Ordering;

pub fn part_one() {
    println!("Hello Day 3 - part 1!");
    let input = super::load_input("input/input-day3");
    let mut output: u32 = 0;
    let banks_input: Vec<&str> = input.trim().split("\n").collect();
    for line in banks_input.iter() {
        let bank = Bank::from_line(line);
        output += bank.max_joltage_part_one();
    }
    println!("Final Output: {output}");
}

pub fn part_two() {
    // println!("Hello Day 3 - part 2!");
    let input = super::load_input("input/input-day3");
    let mut output: u64 = 0;
    let banks_input: Vec<&str> = input.trim().split("\n").collect();
    for line in banks_input.iter() {
        let bank = Bank::from_line(line);
        output += bank.max_joltage_part_two();
    }
    println!("Final Output: {output}");
}

struct Bank {
    batteries: Vec<u32>
}

impl Bank {
    fn from_line(line: &str) -> Self {
        let batteries: Vec<u32> = line.chars().flat_map(|ch| ch.to_digit(10)).collect();
        Self { batteries }
    }

    fn max_joltage_part_one(&self) -> u32 {
        let max_1: (usize, &u32) = self.batteries
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx < &(self.batteries.len() - 1))
            .max_by(|(id0, value0), (id1, value1)|
                match value0.cmp(value1) {
                    Ordering::Equal => {
                        // prefer the lower ids when max value
                        // is equal
                        if id0 > id1  {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    },
                    Ordering::Less => {
                        Ordering::Less
                    },
                    Ordering::Greater => Ordering::Greater,
                }
            )
            .unwrap();
        let max_2: (usize, &u32) = self.batteries
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx > &max_1.0)
            .max_by(|(_, value0), (_, value1)| value0.cmp(value1))
            .unwrap();
        let output = max_1.1 * 10 + max_2.1;
        // println!("Max1: {max_1:?} - Max2: {max_2:?}: {output}");
        output
    }

    fn max_joltage_part_two(&self) -> u64 {
        let capacity = 12;
        let mut max: Vec<(usize, &u32)> = Vec::with_capacity(capacity);
        for _ in 0 ..capacity {
            let list = self.batteries
                .iter()
                .enumerate()
                .filter(|(idx, _)| {
                    if idx < &(self.batteries.len() - capacity + max.len() + 1) {
                        if let Some((last_max_idx, _)) = max.last() {
                            if idx > last_max_idx {
                                return true;
                            }
                        } else {
                            return true;
                        }
                    }
                    return false;
                })
                .max_by(|(id0, value0), (id1, value1)|
                    match value0.cmp(value1) {
                        Ordering::Equal => {
                            // prefer the lower ids when max value
                            // is equal
                            if id0 > id1  {
                                Ordering::Less
                            } else {
                                Ordering::Greater
                            }
                        },
                        Ordering::Less => {
                            Ordering::Less
                        },
                        Ordering::Greater => Ordering::Greater,
                    }
                )
                .unwrap();
            max.push((list.0, list.1));
        }
        let mut output = 0;
        for (n, (_, m)) in max.into_iter().enumerate() {
            output += *m as u64 * (10usize.pow((capacity - n - 1) as u32)) as u64;
        }
        // println!("Output: {output}");
        output
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery() {
        let test_input = 
            "987654321111111
811111111111119
234234234234278
818181911112111";
        let banks_input: Vec<&str> = test_input.trim().split("\n").collect();
        let mut output = 0;
        for line in banks_input.iter() {
            let bank = Bank::from_line(line);
            println!("Bank: {:?}", bank.batteries);
            output += bank.max_joltage_part_two();
        }
        println!("Final Output: {output}");
    }

    #[test]
    fn day3_part2() {
        super::part_two();
    }

}
