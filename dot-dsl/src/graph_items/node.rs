use super::attrs::Attributes;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
  pub id: String,
  pub attrs: HashMap<String, String>,
}

impl Node {
  pub fn new(id: &str) -> Self {
    Node {
      id: id.to_string(),
      attrs: HashMap::new(),
    }
  }

  pub fn with_attrs(self, attributes: &[(&str, &str)]) -> Self {
    let attrs = Attributes::with_attrs(attributes);
    Node { attrs, ..self }
  }

  pub fn get_attr(&self, attr: &str) -> Option<&str> {
    self.attrs.get(attr).map(String::as_str)
  }
}
