#![allow(unused)]
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Range {
    min: u64,
    max: u64
}

impl Range {
    fn in_between(&self, n: u64) -> bool {
        n >= self.min && n <= self.max
    }
    
    fn merge(&self, new_range: &Self) -> Option<Self> {
        if self.min <= new_range.min && self.max >= new_range.max {
            return Some(Self { min: self.min, max: self.max });
        } else if new_range.min <= self.min && new_range.max >= self.max {
            return Some(Self {min: new_range.min, max: new_range.max });
        } else if self.max >= new_range.min && self.max <= new_range.max {
            return Some(Self { min: self.min, max: new_range.max });
        }
        else if self.min <= new_range.max && self.min >= new_range.min {
            return Some(Self {min: new_range.min, max: self.max });
        } else {
            return None;
        }
    }
}

struct Database {
    fresh_ranges: Vec<Range>,
    ids: Vec<u64>
}

impl Database {
    fn from_input(input: &str) -> Self {
        let (part1, part2) = input.split_once("\n\n").expect("Hello wrong input!");
        let mut fresh_ranges = vec![];
        let mut ids = vec![];
        for line in part1.trim().split("\n") {
            if let Some(ranges) = line.split_once("-") {
                fresh_ranges.push(
                    Range { min: ranges.0.parse().unwrap(), max: ranges.1.parse().unwrap()}
                );
            } 
        }
        for line in part2.trim().split("\n") {
            if let Ok(id) = line.parse::<u64>() {
                ids.push(id);
            }
        }

        Self { fresh_ranges, ids }
    }

    fn part_one(&self) -> u32 {
        let mut output = 0u32; 
        for id in self.ids.iter() {
            if self.fresh_ranges.iter().any(|r| r.in_between(*id)) {
                output += 1;
            }
        }
        output
    }

    fn merge_ranges(&mut self) -> bool {
        // first 2 usize are indexes to remove from ranges,
        // last Range type is the new type to push
        let mut merged_ranges_idx: Option<(usize, usize, Range)> = None;
        for (a, r) in self.fresh_ranges.iter().enumerate() {
            for b in a+1..self.fresh_ranges.len() {
                if let Some(new_range) = r.merge(&self.fresh_ranges[b]) {
                    merged_ranges_idx = Some((a, b, new_range));
                } 
            }
        }
        // update the vector of ranges
        if let Some((to_delete_a, to_delete_b, to_add)) = merged_ranges_idx {
            self.fresh_ranges.remove(to_delete_a);
            self.fresh_ranges.remove(to_delete_b - 1);
            self.fresh_ranges.push(to_add);
            self.merge_ranges();
        }
        false
    }

    fn part_two(&mut self) -> u64 {
        let mut output = 0u64;
        self.merge_ranges();
        for r in self.fresh_ranges.iter() {
            output += (r.max - r.min) + 1;
        }
        output
    }
    
}

pub fn part_one() {
    println!("Hello Day 5 - part 1!");
    let input = super::load_input("input/input-day5");
    let db = Database::from_input(&input);
    println!("Output: {}", db.part_one());
}


pub fn part_two() {
    println!("Hello Day 5 - part 2!");
    let input = super::load_input("input/input-day5");
    let mut db = Database::from_input(&input);
    println!("Output: {}", db.part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_database() {
        let test_input = "";
        let mut db = Database::from_input(test_input);
        println!("Ranges: {:?}", db.fresh_ranges);
        db.merge_ranges();
        println!("New Ranges: {:?}", db.fresh_ranges);
        println!("Output: {}", db.part_two());
    }

}
