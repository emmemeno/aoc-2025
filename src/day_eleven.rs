#![allow(dead_code)]

use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::InputMode;

type NodeId = [char; 3];

struct Node {
    id: NodeId,
    output_links: Vec<Rc<RefCell<Node>>>
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
        Self {id, output_links: vec![] }
    }
    fn add_connection(&mut self, new_link: Rc<RefCell<Node>>) {
        self.output_links.push(new_link);
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id   // compare only this field
    }
}

impl Eq for Node {}
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>
}

impl Graph {
    fn new() -> Self {
        Self { nodes: vec![] }

    }

    fn get_node(&self, key: NodeId) -> Option<Rc<RefCell<Node>>> {
        return self.nodes.iter().find_map(|n| {
            if n.borrow().id == key {
                Some(n.clone())
            }  else {
                None
            }
        });
    }

    fn add_node(&mut self, node: Node) -> bool{
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

    fn pathfinding(&self, from: NodeId, to: NodeId, mut current_path: Vec<Rc<RefCell<Node>>>) -> Vec<Rc<RefCell<Node>>>{
        // if no path, add the starting point
        if current_path.len() == 0 {
            current_path.push(self.get_node(from).unwrap());
        }
        // set frontier at last visited path node
        let frontier = current_path.last().unwrap().clone();
        let max_distance = 100;
            if frontier.borrow().id == to {
                println!("Target reached");
                return current_path;
            }
            for child in frontier.borrow().output_links.iter() {
                // println!("From {} to {}", frontier.borrow(), child.borrow());
                // dont visit the same node two times 
                if !current_path.contains(child) {
                    println!("bau");
                    current_path.push(child.clone());
                    current_path = self.pathfinding(from, to, current_path.clone());
                } else {
                    break;
                }
            }
        unreachable!();
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
    let out_node = Node { id: ['o','u','t'], output_links: vec![] };
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
            input =
                "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
".to_string();
        }
        InputMode::Normal => {
            input = super::load_input("input/input-day11");
        }
    }
    generate_graph(input)
}

pub fn part_one() {
    let graph = parse(InputMode::Example);
    println!("{graph}");
    let path = graph.pathfinding(['y','o','u'], ['o','u','t'], vec![]);
    println!("What now?");
    // for n in path {
    //     println!("{}", n.borrow());
    // }

}
