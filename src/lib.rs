use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use serde_derive::{Deserialize, Serialize};

pub enum State {
    Unexplored,
    Visited,
    Finished,
}

#[derive(Debug)]
pub enum GraphError {
    CyclicGraph,
    InvalidDependency,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GraphError::CyclicGraph => write!(f, "In the remake file there are two targets dependent on one another"),
            GraphError::InvalidDependency => write!(f, "In the remake file there is a target dependent on a non-existent target"),
        }
    }
}

impl Error for GraphError {}

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

    pub fn read_from_file(&mut self, path: &str) -> Result<Graph, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let g: Graph = serde_yaml::from_str(&contents)?;
        match g.validate() {
            Some(error) => Err(Box::new(error)),
            None => Ok(g),
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let mut file = File::create(path).expect("Failed to create file");
        let s = serde_yaml::to_string(&self).expect("Failed to serialize graph");
        file.write_all(s.as_bytes()).expect("Failed to write file");
    }

    pub fn insert_edge(
        &mut self,
        target: &str,
        run: &str,
        dependencies: Vec<&str>,
    ) -> Option<GraphError> {
        let info = Component::new(run, dependencies);
        self.make.insert(target.to_string(), info);
        self.validate()
    }

    fn validate(&self) -> Option<GraphError> {
        let mut st = HashMap::new();
        for (_, component) in &self.make {
            for dep in &component.dependencies {
                if self.make.get(dep) == None {
                    return Some(GraphError::InvalidDependency);
                }
            }
            st.insert(component, State::Unexplored);
        }
        let mut cyclic = false;
        for (_, component) in &self.make {
            match *st.get(component).expect("Expected state in all components") {
                State::Unexplored => self.validate_visit(component, &mut st, &mut cyclic),
                State::Visited => {
                    panic!("Validate visit is not expected to leave visited but unfinished graphs")
                }
                State::Finished => (),
            };
        }
        if cyclic == true {
            return Some(GraphError::CyclicGraph);
        }
        None
    }
    fn validate_visit<'a>(
        &'a self,
        component: &'a Component,
        st: &mut HashMap<&'a Component, State>,
        cyclic: &mut bool,
    ) {
        st.insert(component, State::Visited);
        for dep in &component.dependencies {
            let cdep = self.make.get(dep).expect("Validation error");
            match *st.get(cdep).expect("No state") {
                State::Visited => *cyclic = true,
                State::Unexplored => self.validate_visit(cdep, st, cyclic),
                State::Finished => (),
            };
        }
        st.insert(component, State::Finished);
    }
}
