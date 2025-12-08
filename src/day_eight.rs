#![allow(dead_code)]

use std::collections::VecDeque;

#[derive(Clone, Eq, PartialEq, Debug)]
struct JBox {
    x: i64,
    y: i64,
    z: i64
}

impl JBox {
    fn from_str(input: &str) -> Self {
        let xyz: Vec <&str> = input.splitn(3, ",").collect();
        let (x, y, z): (i64, i64, i64) = match &xyz[..] {
            &[first, second, third, ..] => {
                (
                    first.parse().expect("X is not a number"),
                    second.parse().expect("Y is not a number"),
                    third.parse().expect("Z is not a number")
                )
            }
            _ => unreachable!(),
        };
        Self { x, y, z }
    }

     pub fn distance(&self, other: &JBox) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;

        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

enum IsInCircuit<'a> {
    One(&'a JBox),
    Two(&'a JBox, &'a JBox),
    None
}

#[derive(Clone, Debug)]
struct Circuit {
    id: usize,
    links: Vec<JBox>
}

impl Circuit {
    // a circuit start from 2 boxes
    fn new(id: usize, a: &JBox, b: &JBox) -> Self {
        let links = vec![a.clone(), b.clone()];
        Self { id, links }
    }

    // return the added JBox
    fn try_add<'a>(&mut self, a: &'a JBox, b: &'a JBox) -> IsInCircuit<'a> {
        if self.links.contains(&a) && !self.links.contains(&b) {
            self.links.push(b.clone());
            return IsInCircuit::One(b);
        }
        if self.links.contains(&b) && !self.links.contains(&a) {
            self.links.push(a.clone());
            return IsInCircuit::One(a);
        }
        if self.links.contains(&a) && self.links.contains(&b) {
            return IsInCircuit::Two(a,b);
        }
        return IsInCircuit::None;
    }

    fn contains(&self, jbox: &JBox) -> bool {
        self.links.contains(&jbox)
    }

    fn get_size(&self) -> usize {
        self.links.len()
    }

    fn merge(&mut self, other: &Self) {
        self.links.extend(other.links.clone());
    }
}

enum Part {
    One,
    Two
}

fn generate_circuits<'a>(connections: &'a [(f64, &JBox, &JBox)], part: Part) -> Vec<Circuit> {

    // initialized with random value
    let mut last_linked_boxes: (i64, i64) = (0, 0);

    let mut circuits: Vec<Circuit> = vec![];

    // connections
    let available_connections = match part {
        Part::One => &connections[..1000],
        Part::Two => &connections,
    };

    for (_, box_a, box_b) in available_connections {

        // look for bridge link to merge circuits
        let a_is_on_circuit = circuits.iter().cloned().find(|c| c.contains(box_a));
        let b_is_on_circuit = circuits.iter().cloned().find(|c| c.contains(box_b));
        if let Some(a) = a_is_on_circuit && let Some(b) = b_is_on_circuit && a.id!=b.id {
            let mut new_circuit = a.clone();
            new_circuit.merge(&b);
            // remove bridged circuits
            circuits.retain(|c| c.id != a.id && c.id != b.id);
            new_circuit.id = get_new_circuit_id(&circuits);
            circuits.push(new_circuit);
            last_linked_boxes = (box_a.x, box_b.x);
            // println!("Bridged Circuits #{} to #{} with link {:?} {:?}", a.id, b.id, box_a, box_b);
        }
        let mut in_circuit = IsInCircuit::None;
        for n in 0..circuits.len() {
            in_circuit = circuits[n].try_add(box_a, box_b);
            match in_circuit {
                IsInCircuit::One(_) => {
                    last_linked_boxes = (box_a.x, box_b.x);
                    break;
                }
                IsInCircuit::Two(_, _) => break,
                IsInCircuit::None => continue,
            }
        }
        // create a new circuit
        if let IsInCircuit::None = in_circuit {
            // println!("New Circuit {link_a:?}<->{link_b:?}");
            let id = get_new_circuit_id(&circuits);
            circuits.push(Circuit::new(id, box_a, box_b));
        }

    }

    if let Part::Two = part {
        println!("Last Linked Boxes: {last_linked_boxes:?}");
        println!("Output part2: {}", last_linked_boxes.0 * last_linked_boxes.1);
    }
    circuits
}
//very very raw id generator for circuits
fn get_new_circuit_id(circuits: &[Circuit]) -> usize {
    let new_id = if let Some(x) = circuits.last() {
       x.id + 1 
    } else {
        1
    };
    new_id
}
fn get_connections<'a>(boxes: &'a [JBox]) -> Vec<(f64, &'a JBox, &'a JBox)> {

    // store all possible links
    let mut connections: Vec<(f64, &JBox, &JBox) > = Vec::new();
    for (left_n, left_box) in boxes.iter().enumerate() {
        for (_, right_box) in boxes.iter().enumerate().filter(|(right_n, _)| *right_n > left_n) {
            connections.push((left_box.distance(right_box), left_box, right_box));
        }
    }
    // sort link on distance
    connections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap()); 
    return connections;
}

fn parse_boxes() -> Vec<JBox> {
    let input = super::load_input("input/input-day8");
    let lines: Vec<&str> = input.lines().collect();
    let mut boxes = Vec::with_capacity(lines.len());
    for line in lines {
        boxes.push(JBox::from_str(line)); 
    }
    return boxes;
}


pub fn part_one() {

    let boxes = parse_boxes();
    let connections = get_connections(&boxes);

    let mut circuits = generate_circuits(&connections, Part::One);

    // sort circuits by size
    circuits.sort_by(|a, b| b.get_size().cmp(&a.get_size()));

    let mut output = 1usize;
    for circuit in &circuits[..3]{
        let circuit_size = circuit.get_size();
        // println!("Circuit #{} lenght: {} - {:?}", circuit.id, circuit_size, circuit.links);
        println!("Circuit #{} lenght: {}", circuit.id, circuit_size);
        output *= circuit_size;
    }
    println!("\nOutput first part: {}", output);
    
}

pub fn part_two() {

    let boxes = parse_boxes();
    let connections = get_connections(&boxes);

    let _ = generate_circuits(&connections, Part::Two);
}
