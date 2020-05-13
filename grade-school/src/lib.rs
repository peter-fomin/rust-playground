use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Self {roster: BTreeMap::new()}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.entry(grade).or_insert(BTreeSet::new()).insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        Some(self.roster.get(&grade)?.iter().cloned().collect())
    }
}
