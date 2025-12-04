#![allow(dead_code)]

pub fn part_one() {
    println!("Hello Day 4 - part 1!");
    let input = super::load_input("input/input-day4");
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let grid = Grid::from_lines(lines);
    let output = grid.part_one();
    println!("Final Output: {output}");
}

pub fn part_two() {
    println!("Hello Day 4 - part 2!");
    let input = super::load_input("input/input-day4");
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut grid = Grid::from_lines(lines);
    let output = grid.part_two();
    grid.print();
    println!("\nFinal Output: {output}");
}

struct Grid {
    size: (u32, u32),
    values: Vec<bool>
}

impl Grid {
    fn from_lines(lines: Vec<&str>) -> Self {
        let size = (
            lines.first().unwrap().len() as u32,
            lines.len() as u32
        );
        let mut values: Vec<bool> = Vec::with_capacity((size.0 * size.1) as usize); 
        values = lines
            .iter()
            .flat_map(|l| l.chars())
            .collect::<Vec<char>>()
            .iter()
            .map(
                |c| if *c == '@' {
                    true
                } else {
                    false
                })
            .collect();
        // println!("{values:?}");
        Self { size, values }
    }

    // | 0 | 1 | 2 |
    // | 3 | x | 4 |
    // | 5 | 6 | 7 |
    fn get_neighbour_count(&self, pos: (u32, u32)) -> u8 {
        let mut counter = 0;
        let from = (pos.0.saturating_sub(1), pos.1.saturating_sub(1));
        let to = ((pos.0 + 1).clamp(0, self.size.0 - 1), (pos.1 + 1).clamp(0, self.size.1 - 1) );
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                // skip current position
                if x == pos.0 && y == pos.1 {
                    continue
                }
                if let Some(v) = self.values.get((x + self.size.0 * y) as usize) {
                    if *v {
                        counter += 1;
                    }
                }
            }
        }
        counter
    }

    fn part_one(&self) -> u32 {
        let mut output = 0;
        for (n,v) in self.values.iter().enumerate() {
            if *v {
                let (x, y) = (n as u32 % self.size.0, n as u32 / self.size.0);
                let nb_counter = self.get_neighbour_count((x,y));
                if nb_counter < 4 {
                    output += 1;
                } 
            }
        }
        output
    }

    fn print(&self) {

        let mut visual = String::new();
        for (n,v) in self.values.iter().enumerate() {
            if n != 0 && n as u32 % self.size.0 == 0 { 
                visual = format!("{visual}\n");
            }
            if *v {
                let (x, y) = (n as u32 % self.size.0, n as u32 / self.size.0);
                let nb_counter = self.get_neighbour_count((x,y));
                if nb_counter < 4 {
                    visual = format!("{visual} x");
                } else {
                    visual = format!("{visual} @");
                }
            } else {
                visual = format!("{visual} .");
            }
        }
        println!("{visual}");
    }

    fn process(&mut self) -> u32 {
        // same as part one, but consume the values (true -> false)
        let mut output = 0;
        for n in 0..self.values.len() {
            if self.values[n] {
                let (x, y) = (n as u32 % self.size.0, n as u32 / self.size.0);
                let nb_counter = self.get_neighbour_count((x,y));
                if nb_counter < 4 {
                    self.values[n] = false;
                    output += 1;
                } 
            }
        }
        output
    }

    fn part_two(&mut self) -> u32{
        let mut output = 0;
        loop {
           let pass = self.process();
           if pass != 0 {
               output += pass;
           } else {
               break
           }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        let test_input = "";
        let lines: Vec<&str> = test_input.trim().split("\n").collect();
        let mut grid = Grid::from_lines(lines);
        let output = grid.part_two();
        println!("{output}");
    }

}
