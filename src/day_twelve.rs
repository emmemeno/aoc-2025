#![allow(dead_code)]
use super::InputMode;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    cmp::Ordering,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

type ShapeSpace<'a> = [&'a Unit; 9];

#[allow(unused)]
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
    Gap(u8),
    Tooth(u8),
}

#[derive(PartialEq, Eq)]
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
            [Unit::Empty, Unit::Solid(_), Unit::Solid(_)] =>  EdgeKind::Gap(0),
            [Unit::Solid(_), Unit::Empty, Unit::Solid(_)] => EdgeKind::Gap(1),
            [Unit::Solid(_), Unit::Solid(_), Unit::Empty] => EdgeKind::Gap(2),
            [Unit::Empty, Unit::Empty, Unit::Solid(_)] => EdgeKind::Tooth(2),
            [Unit::Empty, Unit::Solid(_), Unit::Empty] => EdgeKind::Tooth(1),
            [Unit::Solid(_), Unit::Empty, Unit::Empty] => EdgeKind::Tooth(0),
            [Unit::Solid(_), Unit::Solid(_), Unit::Solid(_)] => EdgeKind::Full,
        }
    }

    fn priority(&self, edge_side: EdgeSide) -> u8 {
        match self.get_edge(edge_side) {
            EdgeKind::Full => 105,
            EdgeKind::Gap(pos) => {
                if pos == 2 {
                    90
                } else if pos == 1 {
                    75
                }  else {
                    60
                }
            },
            EdgeKind::Tooth(pos) => {
                if pos == 0 {
                    45
                } else if pos == 1 {
                    30
                } else {
                    15
                }
            },
            EdgeKind::Clear => 0,
        }

    }

    fn get_center(&self) -> &Unit {
        &self.units[4]
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

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority(EdgeSide::Left).partial_cmp(&other.priority(EdgeSide::Left))
    }
}
// Shapes are ordered based on full solid left edge...
// ...then top
// ...then the ones with empty center (!) im not sure it does matter
impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.priority(EdgeSide::Left).cmp(&other.priority(EdgeSide::Left)) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                match self.priority(EdgeSide::Top).cmp(&other.priority(EdgeSide::Top)) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => {
                        match (self.get_center(), other.get_center()) {
                            (Unit::Solid(_), Unit::Empty) => Ordering::Less,
                            (Unit::Empty, Unit::Solid(_)) => Ordering::Greater,
                            (Unit::Solid(_), Unit::Solid(_)) | (Unit::Empty, Unit::Empty) => Ordering::Equal,
                        }
                    },
                }
            },
        }
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
}

impl Grid {
    fn new_empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            units: vec![Unit::Empty; width * height],
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

    fn add_shape(&mut self, shape: &Shape, at_pos: (usize, usize)) {
        for (n, unit) in shape.units.iter().enumerate() {
            let shape_pos = ((n % 3), (n / 3));
            let grid_pos = (shape_pos.0 + at_pos.0, shape_pos.1 + at_pos.1);
            if let Unit::Solid(_) = unit {
                self[grid_pos] = unit.clone();
            }
        }
    }
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
        shapes.sort_by(|(_, shape_a), (_, shape_b)| shape_b.cmp(shape_a));
        Self { shapes, counter }
    }
    fn reset_counter(&mut self, new_counter: [u16; 6]) {
        self.counter = new_counter;
    }

    fn left(&self) -> u16 {
        self.counter.iter().sum()
    }

    fn check_space_compatibility(shape: &Shape, space: [&Unit; 9]) -> bool {
        for n in 0..9 {
            if let Unit::Solid(_ ) = shape.units[n] && let Unit::Solid(_) = space[n] {
                return false;
            }
        }
        return true;
    }

    fn get(&mut self, space: [&Unit; 9]) -> Option<&Shape> {
        let mut chosen_one = None;
        for (id, shape) in self.shapes.iter().filter(|(id, _)| self.counter[*id as usize] > 0) {
            if Self::check_space_compatibility(&shape, space) {
                chosen_one = Some(shape);
                self.counter[*id as usize] -= 1;
                break;
            }
        }
        chosen_one
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

pub fn part_one() {
    let (problems, shapes) = parse(InputMode::Normal);
    let mut candidates = Candidates::new(shapes);
    // the output
    let mut ground_complete = 0;


    for (p_id, problem) in problems.into_iter().enumerate() {
        // testing ground
        candidates.reset_counter(problem.required_shapes);
        let (grid_width, grid_height) = problem.grid_size;
        let mut grid = Grid::new_empty(grid_width, grid_height);
        // println!(
        //     "Problem {p_id}: {}-{}, shapes required: {:?}",
        //     problem.grid_size.0, problem.grid_size.1, problem.required_shapes
        // );

        //here should start a loop for problems
        let mut visited_units: Vec<(usize, usize)> = vec![];
        while candidates.left() != 0 {
            let next_unit_to_check = get_next_unit(&grid, &visited_units);
            match next_unit_to_check {
                Some(unit_pos) => {
                    // println!("Visiting {} {}", unit_pos.0, unit_pos.1);
                    let space_to_check = grid.get_shape_space(unit_pos);
                    if let Some(selected_shape) = candidates.get(space_to_check) {
                        grid.add_shape(selected_shape, unit_pos);
                        // println!("Added Shape. Remaining Shapes: {:?}\nNew grid configuration:\n{}", candidates.counter, grid);
                    }

                    visited_units.push(unit_pos);
                }
                None => {
                    // no more explorable units
                    break;
                }
            }
        }
        if candidates.left() == 0 {
            ground_complete += 1;
            println!("Ground {p_id} can fit all presents!");
        } else {
            println!("Ground {p_id} can't. Left items: {:?}", candidates.counter);
        }
    }

    println!("Grounds complete: {ground_complete}");
}

fn parse(mode: InputMode) -> (Vec<Problem>, Vec<Shape>) {
    let input: String;
    match mode {
        InputMode::Example => {
            input = ""
                .to_string();
        }
        InputMode::Normal => input = super::load_input("input/input-day12"),
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
