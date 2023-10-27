use std::collections::HashMap;
use std::fs::File;
use std::io::*;

use serde_derive::{Deserialize, Serialize};

pub enum State {
    Unexplored,
    Visited,
    Finished,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Component {
    pub run: String,
    pub dependencies: Vec<String>,
}
impl Component {
    fn new(run: &str, dependencies: Vec<&str>) -> Component {
        Component {
            run: run.to_string(),
            dependencies: dependencies.iter().map(|s| s.to_string()).collect(),
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Graph {
    pub make: HashMap<String, Component>,
}
impl Graph {
    pub fn new() -> Graph {
        Graph {
            make: HashMap::new(),
        }
    }
    pub fn read_from_file(&mut self, path: &str) -> Option<Graph> {
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file contents");
        let g: Graph = serde_yaml::from_str(&contents).expect("Failed to deserialize graph");
        match g.validate() {
            None => Some(g),
            Some(error) => panic!("{}", error),
        }
    }
    pub fn save_to_file(&self, path: &str) {
        let mut file = File::create(path).expect("Failed to create file");
        let s = serde_yaml::to_string(&self).expect("Failed to serialize graph");
        file.write_all(s.as_bytes()).expect("Failed to write file");
    }
    pub fn insert_edge(&mut self, target: &str, run: &str, dependencies: Vec<&str>) {
        let info = Component::new(run, dependencies);
        self.make.insert(target.to_string(), info);
        if let Some(error) = self.validate() {
            panic!("{}", error);
        }
    }
    fn validate(&self) -> Option<&str> {
        for (target, adj) in &self.make {
            for dep in &adj.dependencies {
                if let None = self.make.get(dep) {
                    return Some("Dependecy doesn't exist");
                }
                if target == dep {
                    return Some("Target can't depend on itself");
                }
            }
        }
        None
    }
}
