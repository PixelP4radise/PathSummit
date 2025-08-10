use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct GraphSettings {
    pub subgroup_vert_num:u8,
    pub num_vert: u16,
    pub num_edges: u16,
    edges_cost: Graph
}

pub fn get_configuration(path: &str) -> GraphSettings {
    let file = File::open(path).expect("Failed to open file.");

    let reader= BufReader::new(file);

    let mut subgroup_vert_num = 0;
    let mut num_vert = 0;
    let mut num_edges = 0;
    let mut edges_cost = Graph {
        edges_cost: HashMap::new()
    };

    for line in reader.lines() {
        let line = line.unwrap();

        let parts:Vec<&str> =line.split_whitespace().collect();

        match parts[0] {
            "k" => {
                subgroup_vert_num = parts[1].parse().unwrap();
            },
            "p" => {
                num_vert = parts[2].parse().unwrap();
                num_edges = parts[3].parse().unwrap();
            }
            "e" => {
                let a = parts[1].parse().unwrap();
                let b = parts[2].parse().unwrap();
                let cost = parts[3].parse().unwrap();
                edges_cost.set_cost(a, b, cost);
            }
            _ => {}
        }
    }

    GraphSettings {
        subgroup_vert_num,
        num_vert,
        num_edges,
        edges_cost
    }
}

type Vertex = u16;
type Cost = u16;

struct Graph {
    edges_cost: HashMap<(Vertex, Vertex), Cost>,
}

impl Graph{
    fn key(a: Vertex, b:Vertex) -> (Vertex, Vertex) {
        if a < b { (a, b) } else { (b, a) }
    }

    fn set_cost(&mut self, a: Vertex, b:Vertex, cost: Cost) {
        self.edges_cost.insert(Self::key(a,b), cost);
    }

    fn get_cost(&self, a:Vertex, b:Vertex) -> Option<&Cost> {
        self.edges_cost.get(&Self::key(a,b))
    }
}