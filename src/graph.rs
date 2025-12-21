use eframe::egui::Pos2;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub note: String,
    pub position: Pos2,
}

impl Node {
    pub fn new(label: String, note: String, position: Pos2) -> Self {
        Self {
            label,
            note,
            position,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub edges: Vec<Edge>,
    pub next_id: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_node(&mut self, label: &str, note: &str, position: Pos2) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes
            .insert(id, Node::new(label.to_string(), note.to_string(), position));
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            self.edges.push(Edge::new(from, to));
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.next_id = 0;
    }
}
