use super::attrs::Attributes;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Edge {
  left: String,
  right: String,
  attrs: HashMap<String, String>,
}

impl Edge {
  pub fn new(a: &str, b: &str) -> Self {
    Edge {
      left: a.to_string(),
      right: b.to_string(),
      attrs: HashMap::new(),
    }
  }

  pub fn with_attrs(self, attributes: &[(&str, &str)]) -> Self {
    let attrs = Attributes::with_attrs(attributes);
    Edge { attrs, ..self }
  }
}
