#![allow(dead_code)]
use super::InputMode;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Unit {
    Solid(char),
    Empty,
}

// NeighBour 4 directions
const NB4: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Problem {
    grid_size: (usize, usize),
    required_shapes: [u16; 6],
}
impl Problem {
    fn from_str(input: &str) -> Self {
        let mut required_shapes = [0; 6];
        let (size_str, reqs) = input.split_once(":").unwrap();
        let (grid_width, grid_height) = size_str.split_once("x").unwrap();
        for (n, r) in reqs.trim().split_whitespace().enumerate() {
            required_shapes[n] = r.parse::<u16>().unwrap();
        }
        // println!("Created ground with size {}-{}", grid_width, grid_height);
        Self {
            grid_size: (grid_width.parse().unwrap(), grid_height.parse().unwrap()),
            required_shapes,
        }
    }
}

type ShapeId = usize;
type ShapeSpace<'a> = [&'a Unit; 9];
type ShapeEdge<'a> = [&'a Unit; 3];

enum EdgeSide {
    Top,
    Right,
    Bottom,
    Left
}
#[derive(Eq, PartialEq)]
enum EdgeKind{
    Full,
    Clear,
    OneGap(u8),
    TwoGap((u8,u8)),
}

impl PartialOrd for EdgeKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
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
        Self { units }
    }

    fn get_edge<'a>(&'a self, side: EdgeSide) -> EdgeKind {
        let edge = match side {
            EdgeSide::Top => [&self.units[0], &self.units[1], &self.units[2]],
            EdgeSide::Right => [&self.units[2], &self.units[5], &self.units[8]],
            EdgeSide::Bottom => [&self.units[6], &self.units[7], &self.units[8]],
            EdgeSide::Left => [&self.units[0], &self.units[3], &self.units[6]],
        };
        match edge {
            [Unit::Empty, Unit::Empty, Unit::Empty] => EdgeKind::Clear,
            [Unit::Empty, Unit::Solid(_), Unit::Solid(_)] =>  EdgeKind::OneGap(0),
            [Unit::Solid(_), Unit::Empty, Unit::Solid(_)] => EdgeKind::OneGap(1),
            [Unit::Solid(_), Unit::Solid(_), Unit::Empty] => EdgeKind::OneGap(2),
            [Unit::Empty, Unit::Empty, Unit::Solid(_)] => EdgeKind::TwoGap((0,1)),
            [Unit::Empty, Unit::Solid(_), Unit::Empty] => EdgeKind::TwoGap((0,2)),
            [Unit::Solid(_), Unit::Empty, Unit::Empty] => EdgeKind::TwoGap((1,2)),
            [Unit::Solid(_), Unit::Solid(_), Unit::Solid(_)] => EdgeKind::Full,
        }
    }

    fn get_unit_char(&self) -> char {
        *self
            .units
            .iter()
            .filter_map(|u| {
                if let Unit::Solid(c) = u {
                    Some(c)
                } else {
                    None
                }
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
        Self { units: new_units }
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
        Self { units: new_units }
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
        Self { units: new_units }
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
        Self { units: new_units }
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
        Self { units: new_units }
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
                Unit::Empty => output = format!("{output}."),
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
        pos.0 < 0 || pos.0 >= self.width as isize || pos.1 < 0 || pos.1 >= self.height as isize
    }

    fn count_empty_neighbour(&self, pos: (usize, usize)) -> u8 {
        let mut counter = 0u8;
        for dir in NB4 {
            let pos_to_check = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
            if self.check_out_of_grid(pos_to_check) {
                continue;
            }
            if let Unit::Empty = self[((pos_to_check.0) as usize, (pos_to_check.1) as usize)] {
                counter += 1;
            }
        }
        counter
    }

    // shape space is a sub grid of 3x3 starting at top left position (pos)
    fn get_shape_space<'a>(&'a self, pos: (usize, usize)) -> ShapeSpace<'a> {
        [
            &self[(pos.0, pos.1)],
            &self[(pos.0 + 1, pos.1)],
            &&self[(pos.0 + 2, pos.1)],
            &self[(pos.0, pos.1 + 1)],
            &self[(pos.0 + 1, pos.1 + 1)],
            &self[(pos.0 + 2, pos.1 + 1)],
            &self[(pos.0, pos.1 + 2)],
            &self[(pos.0 + 1, pos.1 + 2)],
            &self[(pos.0 + 2, pos.1 + 2)],
        ]
    }

    fn get_potential_score(&self) -> u16 {
        self.units.iter().enumerate().fold(0u16, |acc, (n, _)| {
            let potential = self.count_empty_neighbour((n % self.width, n / self.width)) as u16;
            acc + potential
        })
    }

    // this return a map with empty units index and number of its neihbourhood as value
    fn get_units_by_nb_priority(&self) -> Vec<usize> {
        let mut priority: Vec<(usize, u8)> = self
            .units
            .iter()
            .enumerate()
            .filter_map(|(n, u)| {
                let (x, y) = (n % self.width, n / self.width);
                if *u == Unit::Empty && x + 2 < self.width && y + 2 < self.height{
                    let (unit_id, nb_count) = (
                        n,
                        self.count_empty_neighbour((n % self.width, n / self.width)),
                    );
                    Some((unit_id, nb_count))
                } else {
                    None
                }
            })
            .collect();
        priority.sort_by_key(|p| p.1);
        priority.iter().map(|p| p.0).collect()
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
            println!("Error Grid Index: {x}, {y}");
            panic!("Grid: Out of index");
        }
        &self.units[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x > self.width || y > self.height {
            panic!("Grid: Out of index");
        }
        &mut self.units[y * self.width + x]
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

struct Candidates {
    shapes: Vec<(usize, Shape)>,
    counter: [u16; 6],
}
impl Candidates {
    fn new(input_shapes: Vec<Shape>) -> Self {
        let counter = [0; 6];
        let mut shapes: Vec<(usize, Shape)> = vec![];
        for (n, original) in input_shapes.into_iter().enumerate() {
            shapes.push((n, original.flip_h()));
            shapes.push((n, original.flip_v()));
            shapes.push((n, original.rotate_cw()));
            shapes.push((n, original.rotate_ccw()));
            shapes.push((n, original.rotate_180()));
            shapes.push((n, original));
        }
        //todo: Order here based on units
        // prefer left and top edge solid
        // center unit empty
        // right and bottom whatever
        // shapes.sort_by(|a, b| {
        //     let a_left = 
        // });
        Self { shapes, counter }
    }
    fn reset_counter(&mut self, new_counter: [u16; 6]) {
        self.counter = new_counter;
    }

    fn left(&self) -> u16 {
        self.counter.iter().sum()
    }

    fn get(&mut self, space: [&Unit; 9]) -> &Shape {
        todo!()
    }
}

// get the unit_position based on chosen sort function
fn get_next_unit(from_grid: &Grid, exclude: &[(usize, usize)]) -> Option<(usize, usize)> {
    let next_unit_id = from_grid
        .get_units_by_nb_priority()
        .iter()
        .copied()
        .filter_map(|u_id| if !exclude.contains(&(u_id % from_grid.width, u_id / from_grid.width)) {
            Some((u_id % from_grid.width, u_id / from_grid.width))
        } else {
            None
        })
        .next();
    next_unit_id
}

#[allow(unused)]
pub fn part_one() {
    let (mut problems, shapes) = parse(InputMode::Example);
    let mut candidates = Candidates::new(shapes);

    // testing ground
    let problem = problems.remove(1);
    candidates.reset_counter(problem.required_shapes);
    let (grid_width, grid_height) = problem.grid_size;
    let mut grid = Grid::new_empty(grid_width, grid_height);
    println!(
        "Problem 1: {}-{}, shapes required: {:?}",
        problem.grid_size.0, problem.grid_size.1, problem.required_shapes
    );

    //here should start a loop for problems
    let mut visited_units: Vec<(usize, usize)> = vec![];

    while candidates.left() != 0 {
        let next_unit_to_check = get_next_unit(&grid, &visited_units);
        match next_unit_to_check {
            Some(unit_pos) => {
                let space_to_check = grid.get_shape_space(unit_pos);
                println!("Visiting {} {}", unit_pos.0, unit_pos.1);
                visited_units.push(unit_pos);
            }
            None => {
                // no more explorable units
                break;
            }
        }
    }


    // let mut n=0;
    // while grid_units.len() != 0 && n < 50{
    //     n +=1;
    //     let (x, y) = grid_units.first().unwrap();
    //     ground.unit_checked((*x, *y));
    //
    //     let shapes_available = shapes_required
    //         .iter()
    //         .enumerate()
    //         .filter_map(|(n, s)| if *s > 0 { Some((n, &shapes[n])) } else { None })
    //         .collect::<Vec<(usize, &HashSet<Shape>)>>();
    //
    //     let try_to_fit_shape = fit_shape_at_pos(&grid, (*x, *y), &shapes_available);
    //     match try_to_fit_shape {
    //         Some((new_grid, with_shape)) => {
    //             shapes_required[with_shape] -= 1;
    //             grid = new_grid;
    //             println!("Shap {with_shape} fit in grid\n{grid}");
    //         }
    //         None => {
    //             // println!("Cant fit any shape in {x}, {y}");
    //         }
    //     }
    //     if shapes_required.iter().sum::<u8>() == 0 {
    //         println!("Final Configuration:\n{grid}");
    //         break;
    //     }
    //
    //     grid_units = units_to_check(&grid, &ground);
    // }
}

fn parse(mode: InputMode) -> (Vec<Problem>, Vec<Shape>) {
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
    let mut grounds: Vec<Problem> = vec![];
    let mut shapes: Vec<Shape> = vec![];
    let mut blocks: Vec<&str> = input.split("\n\n").collect();
    let grounds_str = blocks.pop().unwrap().lines().collect::<Vec<&str>>();
    let block_visuals = ['A', 'B', 'C', 'D', 'E', 'F'];
    for (n, shape) in blocks.iter().enumerate() {
        let s_line: Vec<&str> = shape.lines().collect();
        shapes.push(Shape::from_str(&s_line[1..], block_visuals[n]));
    }
    for g in grounds_str {
        grounds.push(Problem::from_str(g));
    }
    (grounds, shapes)
}

// return the best grid configuration with selected shape applied
// fn fit_shape_at_pos<'a>(
//     grid: &Grid,
//     pos: (usize, usize),
//     shapes: &[(usize, &HashSet<Shape>)],
// ) -> Option<(Grid, usize)> {
//     // out of grid
//     if pos.0 + 2 >= grid.width || pos.1 + 2 >= grid.height {
//         return None;
//     }
//     let mut configs: Vec<(Grid, ShapeId, u16)> = vec![];
//     for (shape_id, shape_variants) in shapes.iter() {
//         for variant in shape_variants.iter() {
//             let variant_char = variant.get_unit_char();
//             let mut test_grid = (*grid).clone();
//             let mut compatible = true;
//             // check units for solid/solid sovrappositions
//             // and update grid accordingly
//             for (n, shape_unit) in variant.units.iter().enumerate() {
//                 let (x, y) = (n % 3, n / 3);
//                 // checks only on solid units
//                 if *shape_unit == Unit::Empty {
//                     continue;
//                 }
//                 //unit grid is occupied, go to next variant
//                 if let Unit::Solid(_) = test_grid[(pos.0 + x, pos.1 + y)] {
//                     compatible = false;
//                     break;
//                 } else {
//                     test_grid[(pos.0 + x, pos.1 + y)] = Unit::Solid(variant_char);
//                 }
//             }
//             if compatible {
//                 let score = test_grid.get_potential_score();
//                 configs.push((test_grid, *shape_id, score));
//             }
//         }
//     }
//     if configs.is_empty() {
//         return None;
//     }
//     configs.sort_by_key(|(_, _, score)| *score);
//     let best_match = configs.last().unwrap();
//     Some((best_match.0.clone(), best_match.1))
// }
//
// fn units_to_check(updated_grid: &Grid, ground: &Ground) -> Vec<(usize, usize)> {
//     // return the remaining units to check
//     updated_grid.get_units_by_nb_priority()
//         .iter()
//         .filter_map(|u_id|
//             if !ground.checked_units.contains(&u_id) {
//                 Some((u_id % updated_grid.width, u_id / updated_grid.width))
//             } else {
//                 None
//             }
//         )
//         .collect()
// }
