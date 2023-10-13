#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;

type Vertex = usize;
#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    adjacencies: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adjacencies: HashMap::new(),
        }
    }
    pub fn insert_node(&mut self, vertex: Vertex) {
        self.adjacencies.insert(vertex, Vec::new());
    }
    pub fn insert_edge(&mut self, vertex: Vertex, adj: Vertex) {
        if !self.adjacencies.contains_key(&vertex) {
            self.insert_node(vertex);
        }
        if !self.adjacencies.contains_key(&adj) {
            self.insert_node(adj);
        }
        match self.adjacencies.get_mut(&vertex) {
            Some(adj_list) => {
                if !adj_list.contains(&adj) {
                    adj_list.push(adj);
                }
            }
            None => println!("Node not in Graph"),
        }
    }
    pub fn save_to_file(&self, path: &str) {
        let mut file = File::create(path).expect("Failed to open file");
        let yaml = serde_yaml::to_string(self).expect("Failed to serialize graph");
        file.write_all(&yaml.as_bytes())
            .expect("Failed to write to file");
    }
}
pub struct DFSResult {
    pub discovery: HashMap<Vertex, usize>,
    pub finish: HashMap<Vertex, usize>,
    pub topological_sort: Vec<Vertex>,
    pub dag: bool,
}
impl DFSResult {
    fn new() -> DFSResult {
        DFSResult {
            discovery: HashMap::new(),
            finish: HashMap::new(),
            topological_sort: Vec::new(),
            dag: true,
        }
    }
}

pub struct SCCResult {
    pub sccs: Vec<Vec<Vertex>>,
}
impl SCCResult {
    fn new() -> SCCResult {
        SCCResult { sccs: Vec::new() }
    }
}

fn dfs_visit(
    g: &Graph,
    src: &Vertex,
    dfs_result: &mut DFSResult,
    df_time: &mut usize,
    ts_time: &mut usize,
) {
    dfs_result.discovery.insert(*src, *df_time);
    for v in g.adjacencies.get(src).unwrap() {
        if dfs_result.discovery.get(v) == Some(&usize::MAX) {
            *df_time += 1;
            dfs_visit(g, v, dfs_result, df_time, ts_time)
        } else if dfs_result.finish.get(v) == Some(&usize::MAX) {
            dfs_result.dag = false;
        }
    }
    *df_time += 1;
    dfs_result.finish.insert(*src, *df_time);
    dfs_result.topological_sort.insert(*ts_time, *src);
    *ts_time += 1;
}

pub fn dfs(g: &Graph, order: Option<&Vec<Vertex>>) -> DFSResult {
    let mut df_time = 0;
    let mut ts_time = 0;
    let mut dfs_result = DFSResult::new();
    for (vertex, _) in &g.adjacencies {
        dfs_result.discovery.insert(*vertex, usize::MAX);
        dfs_result.finish.insert(*vertex, usize::MAX);
    }
    match order {
        Some(order) => {
            for vertex in order {
                if *dfs_result.discovery.get(vertex).unwrap() == usize::MAX {
                    dfs_visit(g, vertex, &mut dfs_result, &mut df_time, &mut ts_time);
                    df_time += 1;
                }
            }
        }
        None => {
            for (vertex, _) in g.adjacencies.iter() {
                if *dfs_result.discovery.get(vertex).unwrap() == usize::MAX {
                    dfs_visit(g, vertex, &mut dfs_result, &mut df_time, &mut ts_time);
                    df_time += 1;
                }
            }
        }
    }
    dfs_result
}

pub fn bfs(g: &Graph, src: usize) -> Vec<usize> {
    let mut d = Vec::new();
    d.resize(g.adjacencies.len(), usize::MAX);
    d[src] = 0;
    let mut q = VecDeque::new();
    q.push_back(src);
    while !q.is_empty() {
        let u = q.pop_front().expect("queue empty");
        let adj_u = g.adjacencies.get(&u).expect("u not in graph");
        for v in adj_u {
            if d[*v] == usize::MAX {
                d[*v] = d[u] + 1;
                q.push_back(*v)
            }
        }
    }
    d
}

fn transpose(g: &Graph) -> Graph {
    let mut gt = Graph::new();
    for (u, adj) in &g.adjacencies {
        for v in adj {
            gt.insert_edge(*v, *u);
        }
    }
    gt
}

fn get_scc(
    g: &Graph,
    src: Vertex,
    scc_tag: usize,
    visited: &mut HashMap<usize, bool>,
    scc_result: &mut SCCResult,
) {
    visited.insert(src, true);
    scc_result.sccs[scc_tag].push(src);
    for v in g.adjacencies.get(&src).unwrap() {
        if visited.get(v) == Some(&false) {
            get_scc(g, *v, scc_tag, visited, scc_result);
        }
    }
}

pub fn scc(g: &Graph) -> SCCResult {
    let mut scc_result = SCCResult::new();
    let dfs_result = dfs(g, None);
    let mut inv_topological_sort = dfs_result.topological_sort.clone();
    inv_topological_sort.reverse();
    let gt = transpose(g);

    let mut scc_tag = 0;
    let mut visited: HashMap<usize, bool> = HashMap::new();
    for (vertex, _) in &gt.adjacencies {
        visited.insert(*vertex, false);
    }
    for vertex in inv_topological_sort {
        if visited.get(&vertex) == Some(&false) {
            scc_result
                .sccs
                .resize(scc_result.sccs.len() + 1, Vec::new());
            get_scc(&gt, vertex, scc_tag, &mut visited, &mut scc_result);
            scc_tag += 1;
        }
    }

    scc_result
}

pub fn read_from_file(path: &str) -> Graph {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file contents");
    let g = serde_yaml::from_str(&contents).expect("Failed to deserialize graph");
    g
}
