use std::num::ParseIntError;

#[derive(Debug)]
enum Operator {
    Add,
    Mul
}

#[derive(Debug)]
struct Math<'a> {
    rows: Vec<Vec<&'a str>>,
    operators: Vec<Operator>,
    columns_width: Vec<u8>
}

impl<'a> Math<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut rows: Vec<Vec<&str>> = vec![];
        let mut operators = vec![];
        let mut input_lines: Vec<&str> = input.lines().collect();
        println!("Input Lines: {input_lines:?}");
        // last line is operator and column width
        let last_line = input_lines.pop().unwrap();
        let columns_width = get_column_width(last_line);
        let words = last_line.split_whitespace();
        for word in words {
            match word {
                "+" => operators.push(Operator::Add),
                "*" => operators.push(Operator::Mul),
                _ => panic!("Operator {word} not found, wrong input (or parser? :D)"),
            }
        }

        for n in 0..input_lines.len() {
            let mut row_of_cells = vec![];
            let mut new_line = input_lines[n];
            let mut cell = "";
            for (c, col_width) in columns_width.iter().enumerate() {
                // println!("Split row {n} at: {col_width}");
                if c == columns_width.len() - 1 {
                    row_of_cells.push(&new_line[0..]);
                    break
                }
                cell = &new_line[0..(*col_width as usize)];
                row_of_cells.push(cell);
                (_, new_line) = new_line.split_at(*col_width as usize + 1);
                // println!("Cell: -{cell}-, newline: -{new_line}-");
            }
            rows.push(row_of_cells);
        }
       
        println!("{rows:?}");
        Self { rows, operators, columns_width }
    }

    fn part_one(&self) -> u64 {
        let mut output = 0u64;
        let mut row_output = 0u64;
        for (col, o) in self.operators.iter().enumerate() {
            for (pos, value) in self.rows.iter().enumerate() {
                let numerator = match value[col].trim().parse::<u64>() {
                    Ok(x) => x,
                    Err(_) => { panic!("I don't know how to parse {} at row {}, col {}", value[col], pos, col) }
                };
                if pos == 0 {
                    row_output = numerator;
                    continue;
                }
                match o {
                    Operator::Add => {
                        row_output += numerator;
                    },
                    Operator::Mul => {
                        row_output *= numerator;
                    },
                }
            }
            output += row_output;
        }
        output
    }

    fn part_two(&self) -> u64 {
        let mut output = 0u64;
        for (col, width) in self.columns_width.iter().enumerate() {
            let mut col_output = 0u64;
            let mut digits = vec![];
            for c in (0..*width as i32).rev() {
                let mut digit_char = String::new();
                for (r, row) in self.rows.iter().enumerate() {
                    digit_char = format!("{digit_char}{}",row[col].chars().nth(c as usize).expect(&format!("Wrong getting char at column {col} row {r} digit: {c}: Input: {row:?}")));
                }
                digits.push(digit_char.trim().parse::<u64>().expect(&format!("Wrong parsing digit -{digit_char}-")));
            }
            for (n, digit) in digits.iter().enumerate() {
                if n == 0 {
                    col_output = *digit;
                    continue;
                }
                match self.operators[col] {
                    Operator::Add => col_output += digit,
                    Operator::Mul => col_output *= digit,
                }
            }
            output += col_output;
        }
        output
    }
}

fn get_column_width(operator_line: &str) -> Vec<u8> {
    let mut widths = Vec::new();
    let mut count = 1u8;
    let mut in_whitespace = false;

    for c in operator_line.chars() {
        if c == ' '  {
            if in_whitespace {
                // continuing a whitespace run
                count += 1;
            } else {
                // starting a new whitespace run
                in_whitespace = true;
                count = 1;
            }
        } else {
            if in_whitespace {
                // we just finished a whitespace run
                widths.push(count);
                in_whitespace = false;
            }
        }
    }
    // close last column
    if in_whitespace {
        widths.push(count + 1);
    }
    widths
}

pub fn part_one() {

    println!("Hello Day 6 - part 1!");
    let input = super::load_input("input/input-day6");
    let math = Math::from_input(&input);
    println!("Output: {}", math.part_one());
}

pub fn part_two() {

    println!("Hello Day 6 - part 2!");
    let input = super::load_input("input/input-day6");
    let math = Math::from_input(&input);
    println!("Output: {}", math.part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_math_table() {
        let test_input = 
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        // let lines: Vec<&str> = test_input.lines().collect();
        // let splitted = split_at_4(lines.iter().nth(3).unwrap());
        // println!("{splitted:?}");
        let math = Math::from_input(test_input);
        let output = math.part_two();
        println!("Part Two Test Input Output: {output}");
    }

}
