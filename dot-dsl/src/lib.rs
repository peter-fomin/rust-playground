use std::collections::HashMap;

macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (name, value) in attrs {
                self.attrs.insert(name.to_string(), value.to_string());
            }
            self
        }

        pub fn get_attr(&self, attr: &str) -> Option<&str> {
            if let Some(res) = self.attrs.get(attr) {
                Some(&res[..])
            } else {
                None
            }
        }
    };
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n: &&Node| n.name == name)
    }

    impl_attrs!();
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Edge {
    pub start: String,
    pub end: String,
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(start: &str, end: &str) -> Self {
        Self {
            start: String::from(start),
            end: String::from(end),
            attrs: HashMap::new(),
        }
    }

    impl_attrs!();
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            attrs: HashMap::new(),
        }
    }

    impl_attrs!();
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node;
        }
    }
}
