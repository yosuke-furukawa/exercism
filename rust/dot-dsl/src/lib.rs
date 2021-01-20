pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                edges: vec![],
                nodes: vec![],
                attrs: HashMap::new(),
            }
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self
        }
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self
        }
        pub fn get_node(self, node: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name == node).cloned()
        }
    }

    pub mod graph_items {
        use super::*;
        pub mod edge {
            use super::*;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Edge {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
            }
        }
        pub mod node {
            use super::*;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
                pub fn get_attr<'a>(&'a self, key: &str) -> Option<&'a str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }
    }
}
