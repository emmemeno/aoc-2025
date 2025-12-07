#![allow(dead_code)]
use std::collections::HashSet;

// PART ONE
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Beam {
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) {
    let mut split_counter = 0u32;
    let lines: Vec<&str> = input.lines().collect();
    let source_line = lines.first().expect("Empty Input");
    let size = (source_line.len(), lines.len());
    let mut beams = vec![
        Beam { 
            x: source_line.chars().position(|s| s == 'S').unwrap(),
            y: 0
        }
    ];
    // println!("Source Position: {:?}", beams);
    for l in 1..size.1 {
        let mut new_beams = HashSet::new();
        for beam in beams.iter().filter(|b| b.y == l - 1) {
            match lines[l].chars().nth(beam.x).unwrap() {
                '.' => {
                    // println!("Continue the Beam at line {l} from {beam:?}");
                    new_beams.insert(Beam {x: beam.x, y: l});
                }
                '^' => {
                    split_counter +=1;
                    // println!("Split the Beam at line {l} from {beam:?}");
                    new_beams.insert(
                        Beam {
                            x: (beam.x - 1).clamp(0, size.0),
                            y: l
                        }
                    );
                    new_beams.insert(
                        Beam {
                            x: (beam.x + 1).clamp(0, size.0),
                            y: l
                        }
                    );
                }
                _ => { unreachable!() }
            }
        }
        beams.extend(new_beams);
        
    }
    println!("{split_counter}");
}

// PART TWO
struct Tree {
    source_position: usize,
    splits_position: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl Tree {
    fn from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let source_line = lines.first().expect("Empty Input");
        let width = source_line.len();
        let source_position = source_line.chars().position(|s| s == 'S').unwrap();
        let mut splits_position = vec![];
        for line in lines {
            let mut line_splits = vec![];
            for n in line.chars().enumerate().filter_map(
                |(n, c)| { 
                    if c == '^' {
                        Some(n)
                    } else {
                        None
                    }
                }
                ) {
                line_splits.push(n);
            }
            splits_position.push(line_splits);
        }
        // println!("{splits_position:?}");
        Self { source_position, splits_position, height, width }
    }

    fn get_split_at_line(&self, line: usize) -> &[usize] {
        let splits = self.splits_position.get(line).unwrap();
        splits
    }
}


pub fn part_two(input: &str) {
    let tree = Tree::from_str(input);
    let timeline_counter = timelines_count(&tree);
    println!("Part 2. Timeline counter: {timeline_counter}");
}

fn timelines_count(tree: &Tree) -> u64 {
    let mut counter: Vec<u64> = vec![0; tree.width];
    counter[tree.source_position] = 1;
    for l in 0..tree.height {
        let splits = tree.get_split_at_line(l);
        for s in splits {
            if counter[*s] != 0 {
                let old_counter = counter[*s];
                counter[*s] = 0;
                counter[s+1] += old_counter;
                counter[s-1] += old_counter;
            }
        }
    }

    counter.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree() {
        let test_input = 
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        part_two(test_input);
    }

}
