use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    students: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let entry = self.students.entry(grade).or_insert_with(BTreeSet::new);
        entry.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().copied().collect::<Vec<_>>()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(s) = self.students.get(&grade) {
            return Some(s.iter().cloned().collect::<Vec<_>>());
        }

        None
    }
}
