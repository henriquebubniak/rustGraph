use std::collections::HashMap;

type Vertex = usize;
struct Graph {
    adjacencies: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            adjacencies: HashMap::new(),
        }
    }
    fn insert_node(&mut self, vertex: Vertex) {
        self.adjacencies.insert(vertex, Vec::new());
    }
    fn insert_edge(&mut self, vertex: Vertex, adj: Vertex) {
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

struct DFSResult {
    discovery: HashMap<Vertex, usize>,
    finish: HashMap<Vertex, usize>,
    topological_sort: HashMap<Vertex, usize>,
    dag: bool,
}
impl DFSResult {
    fn new() -> DFSResult {
        DFSResult {
            discovery: HashMap::new(),
            finish: HashMap::new(),
            topological_sort: HashMap::new(),
            dag: true,
        }
    }
}

fn dfs_visit(
    g: &Graph,
    src: &Vertex,
    mut dfs_result: DFSResult,
    df_time: &mut usize,
    ts_time: &mut usize,
) -> DFSResult {
    dfs_result.discovery.insert(*src, *df_time);
    for v in g.adjacencies.get(src).unwrap() {
        if dfs_result.discovery.get(v) == Some(&usize::MAX) {
            *df_time += 1;
            dfs_result = dfs_visit(g, v, dfs_result, df_time, ts_time)
        } else if dfs_result.finish.get(v) == Some(&usize::MAX) {
            dfs_result.dag = false;
        }
    }
    *df_time += 1;
    dfs_result.finish.insert(*src, *df_time);
    dfs_result.topological_sort.insert(*src, *ts_time);
    *ts_time += 1;
    dfs_result
}

fn dfs(g: &Graph) -> DFSResult {
    let mut df_time = 0;
    let mut ts_time = 0;
    let mut dfs_result = DFSResult::new();
    for (vertex, _) in &g.adjacencies {
        dfs_result.discovery.insert(*vertex, usize::MAX);
        dfs_result.finish.insert(*vertex, usize::MAX);
    }
    for (vertex, _) in &g.adjacencies {
        if *dfs_result.discovery.get(vertex).unwrap() == usize::MAX {
            dfs_result = dfs_visit(g, vertex, dfs_result, &mut df_time, &mut ts_time);
            df_time += 1;
        }
    }
    dfs_result
}

fn main() {
    let mut graph = Graph::new();
    graph.insert_edge(0, 1);
    graph.insert_edge(1, 2);
    graph.insert_edge(1, 3);
    graph.insert_edge(4, 2);
    graph.insert_edge(2, 5);
    graph.insert_edge(5, 3);
    let result = dfs(&graph);
    for (vertex, discovery_time) in result.discovery {
        println!("Vertex {} was discovered in {}\n", vertex, discovery_time);
    }
    for (vertex, finish_time) in result.finish {
        println!("Vertex {} was finished in {}\n", vertex, finish_time);
    }
    println!("Graph is{} a dag\n", if result.dag { "" } else { " not" });
    for (vertex, top_sort) in result.topological_sort {
        println!(
            "Vertex {} position in topological sort is {}\n",
            vertex, top_sort
        );
    }
}
