use std::collections::HashMap;

pub struct Attributes;

impl Attributes {
  pub fn with_attrs(attributes: &[(&str, &str)]) -> HashMap<String, String> {
    attributes
      .iter()
      .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
      .collect()
  }
}
