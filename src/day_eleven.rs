#![allow(dead_code)]

use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::InputMode;

type NodeId = [char; 3];
type Path = Vec<Rc<RefCell<Node>>>;

struct Node {
    id: NodeId,
    output_links: Vec<Rc<RefCell<Node>>>,
    visited: bool
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format_id = |n: NodeId| format!("{}{}{}", n[0], n[1], n[2]);
        let mut output = format!("{} ->", format_id(self.id));
        for link in self.output_links.iter() {
            output = format!("{} {}", output, format_id((*link).borrow().id));
        }
        write!(f, "{}\n", output)
    }
}

impl Node {
    fn from_str(input: &str) -> Self {
        let (part_one, _) = input.split_once(":").unwrap();
        let id: NodeId = part_one.chars().collect::<Vec<_>>().try_into().unwrap();
        Self {
            id,
            output_links: vec![],
            visited: false,
        }
    }
    fn add_connection(&mut self, new_link: Rc<RefCell<Node>>) {
        self.output_links.push(new_link);
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // compare only this field
    }
}

impl Eq for Node {}
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn get_node(&self, key: NodeId) -> Option<Rc<RefCell<Node>>> {
        return self.nodes.iter().find_map(|n| {
            if n.borrow().id == key {
                Some(n.clone())
            } else {
                None
            }
        });
    }

    fn add_node(&mut self, node: Node) -> bool {
        if self.nodes.iter().any(|n| n.borrow().id == node.id) {
            return false;
        } else {
            self.nodes.push(Rc::new(RefCell::new(node)));
            return true;
        }
    }

    fn add_connections_from_str(&mut self, input_line: &str) -> Result<String, String> {
        let (part_one, part_two) = input_line.split_once(":").unwrap();
        let parent_id: NodeId = part_one.chars().collect::<Vec<_>>().try_into().unwrap();
        let parent_node = if let Some(n) = self.get_node(parent_id) {
            n
        } else {
            return Result::Err("Parent node {part_one} not found".to_string());
        };

        for link in part_two.trim().split(" ") {
            let child_id: NodeId = link.chars().collect::<Vec<_>>().try_into().unwrap();
            let child_node = if let Some(n) = self.get_node(child_id) {
                n
            } else {
                return Err("Child node {link} not found".to_string());
            };
            parent_node.borrow_mut().add_connection(child_node.clone());
        }
        return Ok("Connections created".to_string());
    }

    fn pathfinding(
        &self,
        from: NodeId,
        to: NodeId,
        mut counter: u64,
    ) -> u64 {
        let frontier = self.get_node(from).unwrap();
        if frontier.borrow().id == to {
            print!(".");
            return counter + 1;
        }
        for child in frontier.borrow().output_links.iter() {
            counter = self.pathfinding(child.borrow().id, to, counter);
        }
    counter
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for node in self.nodes.iter() {
            output = format!("{}{}", output, node.borrow());
        }
        write!(f, "{}", output)
    }
}

fn generate_graph(input: String) -> Graph {
    // first create all nodes:
    let out_node = Node {
        id: ['o', 'u', 't'],
        output_links: vec![],
        visited: false
    };
    let mut graph = Graph::new();
    graph.add_node(out_node);
    for line in input.lines() {
        graph.add_node(Node::from_str(line));
    }
    // create connections
    for line in input.lines() {
        _ = graph.add_connections_from_str(line);
    }
    graph
}

fn parse(mode: InputMode) -> Graph {
    let input: String;
    match mode {
        InputMode::Example => {
            input = "".to_string();
        }
        InputMode::Normal => {
            input = super::load_input("input/input-day11");
        }
    }
    generate_graph(input)
}

pub fn part_one() {
    let graph = parse(InputMode::Normal);
    // println!("{graph}");
    let output = graph.pathfinding(['y', 'o', 'u'], ['o', 'u', 't'], 0);
    // for path in paths.iter() {
    //     for node in path.iter() {
    //         println!("{}", node.borrow());
    //     }
    // }
    println!("Part One Output: {}", output);
}

pub fn part_two() {
    let graph = parse(InputMode::Normal);
    // println!("{graph}");
    // let output_1 = graph.pathfinding(['s', 'v', 'r'], ['f', 'f', 't'], 0);
    // println!("SVR to FFT: {}", output_1);
    // let output_2 = graph.pathfinding(['f', 'f', 't'], ['d', 'a', 'c'], 0);
    // println!("FFT to DAC to Out: {}", output_2);
    let output_3 = graph.pathfinding(['d', 'a', 'c'], ['o', 'u', 't'], 0);
    println!("DAC to OUT: {}", output_3);
}
