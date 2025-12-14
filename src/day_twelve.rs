#![allow(dead_code)]
use super::InputMode;
use std::{fmt::Display, ops::{Index, IndexMut, Not}};
use std::collections::HashSet;

// Some random thought about this problem
// I could assign a score potential to a grid configuration based on empty positions '.' and its neighbour
// calculate the sum of empty neihbours for each empty space. For example a grid:
// ..
// ..
// has a score of 8 becuase each space have 2 empty neighbours. The grid:
// ...
// ...
// ...
// has a score of 24 (from top left to bottom right: ( 2 + 3 + 2 + 3 + 4 + 3 + 2 + 3 + 2)
// // Again:
// ..#
// ...
// .#.
// has a score of 13 (2 + 2 + 0 + 3 + 3 + 1 + 1 + 0 + 1)
//
// OR....
// Use the edge of shapes like a jigsaw puzzle

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Unit {
    Solid(char),
    Empty,
}

// NeighBour 4 directions
const NB4: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

type EdgeUnits<'a> = [&'a Unit; 3];
enum EdgeSide {
    Top,
    Right,
    Bottom,
    Left
}

// !EdgeSide is the opposite edge
impl Not for EdgeSide {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
        }
    }
}

// gap position is one-dimensional
// from left (best) to right (worst)
// from top (best) to bottom (worst)
enum MatchScore {
    Incompatible,
    PerfectFit,
    WithGap{ gap_pos: u8 }
}
#[derive(Hash, PartialEq, Eq)]
struct Shape {
    units: [Unit; 9],
}

impl Shape {
    fn from_str(input: &[&str], display_char: char) -> Self {
        let mut units = [Unit::Empty; 9];
        let chars = input.iter().flat_map(|i| i.chars());
        for (n, c) in chars.enumerate() {
            match c {
                '#' => units[n] = Unit::Solid(display_char),
                '.' => units[n] = Unit::Empty,
                _ => panic!(),
            }
        }
        Self { units}
    }

    fn get_unit_char(&self) -> char {
        *self.units.iter()
            .filter_map(|u|
                if let Unit::Solid(c) = u {
                    Some(c)
                } else {
                    None
                })
            .next()
            .unwrap()
    }

    // Flip Horizontal
    // 2 1 0
    // 5 4 3
    // 8 7 6
    fn flip_h(&self) -> Self {
        let mut new_units = [Unit::Empty; 9];
        new_units[0] = self.units[2];
        new_units[1] = self.units[1];
        new_units[2] = self.units[0];
        new_units[3] = self.units[5];
        new_units[4] = self.units[4];
        new_units[5] = self.units[3];
        new_units[6] = self.units[8];
        new_units[7] = self.units[7];
        new_units[8] = self.units[6];
        Self {
            units: new_units
        }
    }

    // Flip Vertical
    // 6 7 8
    // 3 4 5
    // 0 1 2
    fn flip_v(&self) -> Self {
        let mut new_units = [Unit::Empty; 9];
        new_units[0] = self.units[6];
        new_units[1] = self.units[7];
        new_units[2] = self.units[8];
        new_units[3] = self.units[3];
        new_units[4] = self.units[4];
        new_units[5] = self.units[5];
        new_units[6] = self.units[0];
        new_units[7] = self.units[1];
        new_units[8] = self.units[2];
        Self {
            units: new_units,
        }
    }

    // Rotate Clockwise
    // 6 3 0
    // 7 4 1
    // 8 5 2
    fn rotate_cw(&self) -> Self {
        let mut new_units = [Unit::Empty; 9];
        new_units[0] = self.units[6];
        new_units[1] = self.units[3];
        new_units[2] = self.units[0];
        new_units[3] = self.units[7];
        new_units[4] = self.units[4];
        new_units[5] = self.units[1];
        new_units[6] = self.units[8];
        new_units[7] = self.units[5];
        new_units[8] = self.units[2];
        Self {
            units: new_units,
        }
    }

    // Rotate CounterClockwise
    // 2 5 8
    // 1 4 7
    // 0 3 6
    fn rotate_ccw(&self) -> Self {
        let mut new_units = [Unit::Empty; 9];
        new_units[0] = self.units[2];
        new_units[1] = self.units[5];
        new_units[2] = self.units[8];
        new_units[3] = self.units[1];
        new_units[4] = self.units[4];
        new_units[5] = self.units[7];
        new_units[6] = self.units[0];
        new_units[7] = self.units[3];
        new_units[8] = self.units[6];
        Self {
            units: new_units,
        }
    }

    // Rotate 180 aka Flip Horizontal AND Vertical
    // 8 7 6
    // 5 4 3
    // 2 1 0
    fn rotate_180(&self) -> Self {
        let mut new_units = [Unit::Empty; 9];
        new_units[0] = self.units[8];
        new_units[1] = self.units[7];
        new_units[2] = self.units[6];
        new_units[3] = self.units[5];
        new_units[4] = self.units[4];
        new_units[5] = self.units[3];
        new_units[6] = self.units[2];
        new_units[7] = self.units[1];
        new_units[8] = self.units[0];
        Self {
            units: new_units,
        }
    }

}

// access the grid with Grid[(x, y)]
impl Index<(usize, usize)> for Shape {
    type Output = Unit;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x >= 3 || y >= 3 {
            panic!("Shape: Out of index");
        }
        &self.units[x * 3 + y]
    }
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        for (n, unit) in self.units.iter().enumerate() {
            if n % 3 == 0 {
                output.push_str("\n");
            }
            match unit {
                Unit::Solid(c) => output = format!("{output}{}", c),
                Unit::Empty => output = format!("{output}." ),
            }
        }
        write!(f, "{}", output)
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    units: Vec<Unit>,
    display: (char, char),
}

impl Grid {
    fn new_empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            units: vec![Unit::Empty; width * height],
            display: ('#', '.'),
        }
    }

    fn iter(&self) -> GridIterator<'_> {
        GridIterator {
            grid: self,
            index: 0,
        }
    }

    fn check_out_of_grid(&self, pos: (isize, isize)) -> bool {
        println!("Checking pos {}.{} for grid: {}.{}", pos.0, pos.1, self.width,self.height);
        pos.0 < 0 || pos.0 >= self.width as isize || pos.1 < 0 || pos.1 >= self.height as isize
    }

    fn count_empty_neighbour(&self, pos: (usize, usize)) -> u8 {
        let mut counter = 0u8;
        for dir in NB4 {
            let pos_to_check = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
            if self.check_out_of_grid(pos_to_check) {
                println!("Pos {}-{} out of grid",pos_to_check.0, pos_to_check.1);
                continue;
            }
            if let Unit::Empty = self[((pos_to_check.0) as usize, (pos_to_check.1) as usize)] 
            {
                counter += 1;
            }
        }
        counter
    }
    fn get_potential_score(&self) -> u16 {
        self.units.iter().enumerate().fold(0u16, |acc, (n, _)| {
            let potential = self.count_empty_neighbour((n % self.width, n / self.width)) as u16;
            acc + potential
        })
    }

    // fn apply_shape(&mut self, shape: &Shape, at_pos: (usize, usize)) {
    //     for (n, unit) in shape.units.iter().enumerate() {
    //         let shape_pos = ((n % 3), (n / 3));
    //         let grid_pos = (shape_pos.0 + at_pos.0, shape_pos.1 + at_pos.1);
    //         self[grid_pos] = unit.clone();
    //     }
    // }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        for (n, unit) in self.units.iter().enumerate() {
            if n % self.width == 0 && n != 0 {
                output.push_str("\n");
            }
            match unit {
                Unit::Solid(c) => output = format!("{output}{c}"),
                Unit::Empty => output = format!("{output}."),
            }
        }
        write!(f, "{}", output)
    }
}

struct GridIterator<'a> {
    grid: &'a Grid,
    index: usize,
}

// access the grid with Grid[(x, y)]
impl Index<(usize, usize)> for Grid {
    type Output = Unit;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x >= self.width || y >= self.height {
            panic!("Grid: Out of index");
        }
        &self.units[x * self.width + y]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x >= self.width || y >= self.height {
            panic!("Grid: Out of index");
        }
        &mut self.units[x * self.width + y]
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a Unit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.units.len() {
            let result = Some(&self.grid.units[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

struct Ground {
    grid: Grid,
    required_shapes: [u8; 6],
}
impl Ground {
    fn from_str(input: &str) -> Self {
        let mut required_shapes = [0; 6];
        let (size_str, reqs) = input.split_once(":").unwrap();
        let (grid_width, grid_height) = size_str.split_once("x").unwrap();
        for (n, r) in reqs.trim().split_whitespace().enumerate() {
            required_shapes[n] = r.parse::<u8>().unwrap();
        }
        let grid = Grid::new_empty(grid_width.parse().unwrap(), grid_height.parse().unwrap());
        // println!("Created ground with size {}-{}", grid_width, grid_height);
        Self {
            grid,
            required_shapes,
        }
    }

}

fn parse(mode: InputMode) -> (Vec<Ground>, Vec<Shape>) {
    let input: String;
    match mode {
        InputMode::Example => {
            input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
                .to_string();
        }
        InputMode::Normal => todo!(),
    }
    let mut grounds: Vec<Ground> = vec![];
    let mut shapes: Vec<Shape> = vec![];
    let mut blocks: Vec<&str> = input.split("\n\n").collect();
    let grounds_str = blocks.pop().unwrap().lines().collect::<Vec<&str>>();
    let block_visuals = ['A', 'B', 'C', 'D', 'E', 'F'];
    for (n, shape) in blocks.iter().enumerate() {
        let s_line: Vec<&str> = shape.lines().collect();
        shapes.push(Shape::from_str(&s_line[1..], block_visuals[n]));
    }
    for g in grounds_str {
        grounds.push(Ground::from_str(g));
    }
    (grounds, shapes)
}

fn shape_variants(original: Shape) -> HashSet<Shape> {
    let mut variants = HashSet::new();
    variants.insert(original.flip_h());
    variants.insert(original.flip_v());
    variants.insert(original.rotate_cw());
    variants.insert(original.rotate_ccw());
    variants.insert(original.rotate_180());
    variants.insert(original);
    variants
}

// return the best grid configuration with shape applied
fn arrange_shape_at_pos<'a>(grid: &Grid, pos: (usize, usize), shape_variants: &'a HashSet<Shape>) -> Option<Grid> {
    // out of grid
    if pos.0 + 2 > grid.width || pos.1 + 2 > grid.height {
        return None
    }
    let mut configs: Vec<(Grid, u16)> = vec![];
    for variant in shape_variants.iter() {
        let variant_char = variant.get_unit_char();
        let mut test_grid = (*grid).clone();
        let mut compatible = true;
        // check units for solid/solid sovrappositions
        // and update grid accordingly
        for (n, shape_unit) in variant.units.iter().enumerate() {
            let (x, y) = (n % 3, n / 3);
            // checks only on solid units
            if *shape_unit == Unit::Empty {
                continue;
            }
            //unit grid is occupied, go to next variant
            if let Unit::Solid(_) = test_grid[(pos.0 + x, pos.1 + y)] {
                compatible = false;
                break;
            } else {
                test_grid[(pos.0 + x, pos.1 + y)] = Unit::Solid(variant_char) ;
            }
        }
        if compatible {
            let score = test_grid.get_potential_score();
            configs.push((test_grid, score));
        }
    }
    if configs.is_empty() {
        return None
    }
    println!("Debug available variants: {}", configs.len());
    configs.sort_by_key(|(_, score)| *score );
    Some(configs.last().unwrap().0.clone())
}

pub fn part_one() {
    let (grounds, shapes) = parse(InputMode::Example);
    let shapes = shapes.into_iter().map(|s| shape_variants(s)).collect::<Vec<HashSet<Shape>>>();
    // testing
    let test_grid = grounds[1].grid.clone();
    let on_first_shape = arrange_shape_at_pos(&test_grid, (0, 0), &shapes[4]);
    println!("First Shape");
    let test_grid = match on_first_shape {
        Some(new_grid) => {
            println!("{new_grid}");
            new_grid
        }
        None =>  {
            panic!("fail to apply first shape");
        }
    };
    let on_second_shape = arrange_shape_at_pos(&test_grid, (1, 1), &shapes[4]);
    println!("Second Shape");
    let test_grid = match on_second_shape {
        Some(new_grid) => {
            println!("{new_grid}");
            new_grid
        }
        None =>  {
            panic!("fail to apply second shape");
        }
    };
}
