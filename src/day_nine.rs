#![allow(dead_code)]

use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Color {
    Red,
    Green
}

#[derive(Clone, Debug)]
struct Tile {
    x: u32,
    y: u32,
    color: Color
}

impl Tile {
    fn from_str(input: &str, color: Color) -> Self {
        let (x, y) = input.split_once(",").expect("Wrong line format");
        let x = x.parse().expect("X is not a number");
        let y = y.parse().expect("Y is not a number");
        Self { x, y, color }
    }
}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Tile {}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        // first compare y, then x
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            other => other,
        }
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) 
    }
}

type Rectangle = (u32, u32, u32, u32);

struct Floor {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl Floor {
    fn from_red_tiles(red_tiles: Vec<Tile>) -> Self {
        let width = red_tiles.iter().max_by_key(|t| t.x).unwrap().x;
        let height = red_tiles.iter().max_by_key(|t| t.y).unwrap().y;
        let mut tiles: Vec<Tile> = vec![];
        let mut last_red_tile = red_tiles.last().unwrap();
        for red in red_tiles.iter() {
            tiles.push(red.clone());
            // link vertically two red tiles with green ones
            if red.x == last_red_tile.x {
                let from_y = std::cmp::min(red.y, last_red_tile.y);
                let to_y = std::cmp::max(red.y, last_red_tile.y);
                for y in from_y+1..to_y {
                    tiles.push(Tile {x: red.x, y, color: Color::Green});
                }
            // and horizontally
            } else if red.y == last_red_tile.y {
                let from_x = std::cmp::min(red.x, last_red_tile.x);
                let to_x = std::cmp::max(red.x, last_red_tile.x);
                for x in from_x+1..to_x {
                    tiles.push(Tile {x, y: red.y, color: Color::Green});
                }
            }
            last_red_tile = red;
        }
        // order tiles on y
        // tiles.sort();

        // let mut last_tile = &tiles[0];
        // let mut filled_area: Vec<Tile> = vec![];
        // // fill the tiles
        // println!("Tiles len so far: {}", tiles.len());
        // for n in 1..tiles.len() {
        //     // a previous tile was present in the same line, so fill the gap!
        //     if tiles[n].y == last_tile.y {
        //         for x in last_tile.x+1..tiles[n].x {
        //
        //             // filled_area.push(Tile {x, y: last_tile.y, color: Color::Green});
        //         }
        //     }
        //     last_tile = &tiles[n];
        // }
        // tiles.extend(filled_area);
        // for y in 0..height {
        //     let mut tile_line = tiles.iter().filter(|t| t.y == y).map(|t| t.x).collect::<HashSet<u32>>();
        //     let mut filling = false;
        //     for x in 0..width {
        //         if tile_line.remove(&x) {
        //             filling = true;
        //             if tile_line.len() == 0 {
        //                 filling = false;
        //             }
        //
        //         }
        //         if filling && !tiles.iter().any(|t| t.x == x && t.y == y) {
        //             tiles.push(Tile {x, y, color: Color::Green});
        //         }
        //     }
        // }
        Self {width, height, tiles }
    }
    fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.x == x && t.y == y) 
    }

    fn get_tiles_in_area(&self, from_tile: (u32, u32), to_tile: (u32, u32)) -> Vec<&Tile> {
        let min_x = std::cmp::min(from_tile.0, to_tile.0);
        let max_x = std::cmp::max(from_tile.0, to_tile.0);
        let min_y = std::cmp::min(from_tile.1, to_tile.1);
        let max_y = std::cmp::max(from_tile.1, to_tile.1);
        self.tiles.iter().filter(|t| t.x >= min_x && t.x <= max_x && t.y >= min_y && t.y <= max_y).collect::<Vec<&Tile>>()
    }

    fn check_intersection(&self, tile_a: &Tile, tile_b: &Tile) -> bool {
       for y in tile_a.y..=tile_b.y {
           for x in tile_a.x..=tile_a.x {
               if self.tiles.iter().any(|t| t.x == x && t.y == y) {
                   return true;
               }
           }
       }
       return false;
    }

    fn largest_area(&self) -> u64 {

        let mut areas: Vec<(u64, &Tile, &Tile)> = vec![];
        let red_tiles = self.tiles.iter().filter(|t| t.color == Color::Red).collect::<Vec<&Tile>>();
        for (n, tile_a) in red_tiles.iter().enumerate() {
           for (_, tile_b) in red_tiles.iter().enumerate().filter(|(i, _)| *i > n) {
               areas.push((calc_area(tile_a, tile_b), &tile_a, &tile_b));
           }
        }
        areas.sort_by_key(|k| k.0);
        // if area doest not intersect the edges im good
        for (v, a, b) in areas.into_iter().rev() {
            if self.check_intersection(a, b) {
                return v;
            }
        }
        unreachable!()
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = String::new();
        for y in 0..=self.height {
            let mut line = String::new();
            for x in 0..=self.width {
                let o = if let Some(tile) = self.get_tile(x,y) {
                    match tile.color {
                        Color::Red => '#',
                        Color::Green => 'X',
                    }
                } else {
                    '.'
                };
                line.push(o);
            }
            lines = format!("{lines}\n{line}");
        }
        write!(f, "{}", lines)
    }
}
fn parse_red_tiles(mode: super::InputMode) -> Vec<Tile> {
    let input: String;
    match mode {
        crate::InputMode::Example => {
            input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3".to_string();
        }
        crate::InputMode::Normal => {
            input = super::load_input("input/input-day9");
        }
    }
    let lines: Vec<&str> = input.lines().collect();
    lines.into_iter().map(|l| Tile::from_str(l, Color::Red)).collect()
}

fn calc_area(a: &Tile, b: &Tile) -> u64 {
    let l = (a.x as i64 - b.x as i64).abs() + 1;
    let w = (a.y as i64 - b.y as i64).abs() + 1;
    (l * w) as u64
}

pub fn part_one() {
    let tiles = parse_red_tiles(super::InputMode::Normal);
    let mut areas: Vec<u64> = vec![];
    for (n, tile_a) in tiles.iter().enumerate() {
       for (_, tile_b) in tiles.iter().enumerate().filter(|(i, _)| *i > n) {
           areas.push(calc_area(tile_a, tile_b));
       }
    }
    areas.sort();
    let max_area = areas.last().unwrap();
    println!("Max Area: {max_area}");
}

pub fn part_two() {
    let red_tiles = parse_red_tiles(super::InputMode::Example);
    let floor = Floor::from_red_tiles(red_tiles);
    // println!("{floor}");
    // let tiles_in_area = floor.get_tiles_in_area((2,3), (9,5));
    // println!("Tiles in area: {tiles_in_area:?}");
    let largest_area = floor.largest_area();
    println!("Largest area: {largest_area}");
}
