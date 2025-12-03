#![allow(dead_code)]

pub fn part_one() {
    println!("Hello Day Two - part one");
    let mut output = 0usize;
    let input = super::load_input("input/input-day2");
    let ranges: Vec<&str> = input.trim().split(",").collect();
    for range in ranges {
        let n = IdRange::from_str(range);
        output += n.find_repetitions_part_one().iter().sum::<usize>();
    }
    println!("Result: {output}");
}

pub fn part_two() {
    println!("Hello Day Two - part two");
    let mut output = 0usize;
    let input = super::load_input("input/input-day2");
    let ranges: Vec<&str> = input.trim().split(",").collect();
    for range in ranges {
        let n = IdRange::from_str(range);
        output += n.find_repetitions_part_two();
    }
    println!("Result: {output}");
}

struct IdRange {
    start: usize,
    end: usize,
}

impl IdRange {
    fn from_str(input: &str) -> Self {
        let (l, r) = input.split_once("-").unwrap();
        let (start, end) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());
        Self { start, end }
    }

    fn find_repetitions_part_one(&self) -> Vec<usize> {
        let mut output = vec![];
        for x in self.start..=self.end {
            let x_string = x.to_string();
            let (a, b) = x_string.split_at(x_string.len() / 2);
            if a == b {
                output.push(format!("{a}{b}").parse::<usize>().unwrap());
            }
        }
        output
    }

    fn find_repetitions_part_two(&self) -> usize {
        let mut output = 0usize;
        for x in self.start..=self.end {
            let n = check_id(x.to_string());
            output += n.iter().sum::<usize>();
        }
        output
    }


}

fn check_id(id: String) -> Vec<usize>{

    // println!("Checking ID {id}");
    let mut output = vec![];
    for i in (1 ..=id.len()/2 ).rev() {
        let chunks = id.chars()
            .collect::<Vec<char>>()
            .chunks(i)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>();
        let comparison = chunks.first().unwrap();
        // println!("..substr to compare: {}", comparison);
        let mut condition = true;
            for chunk in chunks.iter() {
                if chunk != comparison {
                    condition = false;
                    break;
                }
            }
        if condition {
            let new_found = comparison.repeat(chunks.len()).parse::<usize>().unwrap();
            // println!("...found repetition: {}", new_found);
            output.push(new_found);
            break;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reps() {
        let test_input = "";
        // let test_input = "222220-222224";
        let ranges: Vec<&str> = test_input.trim().split(",").collect();
        let mut output = 0usize;
        for range in ranges {
            let n = IdRange::from_str(range);
            output += n.find_repetitions_part_two();
        }
        println!("Output: {output}");
    }

    #[test]
    fn single() {
        let test_id = "1212".to_string();
        check_id(test_id);
    }
}
