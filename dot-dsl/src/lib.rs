pub mod graph_items;

pub mod graph {
    use crate::graph_items::attrs::Attributes;
    pub use crate::graph_items::{self, edge, node};
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<node::Node>,
        pub edges: Vec<edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn with_nodes(self, nodes: &[node::Node]) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }
        pub fn with_edges(self, edges: &[edge::Edge]) -> Self {
            Graph {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attributes: &[(&str, &str)]) -> Self {
            let attrs = Attributes::with_attrs(attributes);
            Graph { attrs, ..self }
        }

        pub fn get_node(self, id: &str) -> Option<node::Node> {
            self.nodes.into_iter().find(|node| node.id == id)
        }
    }
}
