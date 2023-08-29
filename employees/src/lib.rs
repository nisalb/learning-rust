use crate::operations::Operation;
use std::collections::{HashMap, HashSet};

pub mod operations;

pub struct Company {
    dep_map: HashMap<String, HashSet<String>>,
}

impl Company {
    pub fn new() -> Self {
        Company {
            dep_map: HashMap::new(),
        }
    }

    pub fn operation(&mut self, command: &str) -> Option<Operation> {
        operations::parse_op(command)
    }

    pub fn list_all(&self, department: &String) -> Vec<String> {
        match self.dep_map.get(department) {
            Some(set) => set.into_iter().map(|e| e.clone()).collect(),
            None => Vec::new(),
        }
    }

    pub fn add(&mut self, person: String, department: String) {
        let employees = self.get_department(department);
        employees.insert(person);
    }

    pub fn remove(&mut self, person: &String, department: String) {
        let employees = self.get_department(department);
        employees.remove(&person[..]);
    }

    pub fn transfer(&mut self, person: String, from: String, to: String) {
        self.remove(&person, from);
        self.add(person, to);
    }

    fn get_department(&mut self, department: String) -> &mut HashSet<String> {
        self.dep_map.entry(department).or_insert(HashSet::new())
    }
}
