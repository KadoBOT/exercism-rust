use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct School {
    students: HashMap<u32, HashSet<String>>,
}

impl School {
    pub fn new() -> School {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let entry = self.students.entry(grade).or_insert_with(HashSet::new);
        entry.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self
            .students
            .iter()
            .map(|(&grade, _)| grade)
            .collect::<Vec<_>>();
        result.sort();
        result
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(s) = self.students.get(&grade) {
            let mut result = s.iter().cloned().collect::<Vec<_>>();
            result.sort();
            return Some(result);
        }

        None
    }
}
