#[derive(Debug)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for item in other.data.iter() {
            if !self.data.contains(item) {
                return false;
            }
        }
        true
    }
}

impl<'a, T: PartialEq + std::clone::Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        CustomSet {
            data: input.to_vec(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        for item in &self.data {
            if item == element {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        match (self.data.len(), other.data.len()) {
            (0, _) => true,
            (_, 0) => false,
            _ if self.data == other.data => true,
            _ if other.data.windows(self.data.len()).any(|d| d == self.data) => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        match (self.data.len(), other.data.len()) {
            (0, _) | (_, 0) => true,
            _ if other.data.iter().any(|d| self.data.contains(d)) => false,
            _ => true,
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .cloned()
            .filter(|a| other.contains(a))
            .collect::<Vec<_>>();
        CustomSet { data }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .cloned()
            .filter(|a| !other.contains(a))
            .collect::<Vec<_>>();
        CustomSet { data }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut data = vec![];
        let mut add_data = |d: Vec<T>| {
            for item in d {
                if !data.contains(&item) {
                    data.push(item)
                }
            }
        };
        add_data(self.data.clone());
        add_data(other.data.clone());
        CustomSet { data }
    }
}
