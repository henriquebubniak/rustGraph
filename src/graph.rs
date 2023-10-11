#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

type Vertex = usize;
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
            Some(adj_list) => adj_list.push(adj),
            None => println!("Node not in Graph"),
        }
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

pub fn dfs(g: &Graph) -> DFSResult {
    let mut df_time = 0;
    let mut ts_time = 0;
    let mut dfs_result = DFSResult::new();
    for (vertex, _) in &g.adjacencies {
        dfs_result.discovery.insert(*vertex, usize::MAX);
        dfs_result.finish.insert(*vertex, usize::MAX);
    }
    for (vertex, _) in g.adjacencies.iter() {
        if *dfs_result.discovery.get(vertex).unwrap() == usize::MAX {
            dfs_visit(g, vertex, &mut dfs_result, &mut df_time, &mut ts_time);
            df_time += 1;
        }
    }
    dfs_result
}

pub fn bfs(g : &Graph, src : usize) -> Vec<usize> {
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