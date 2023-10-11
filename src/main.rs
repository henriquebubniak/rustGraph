mod graph;
use crate::graph::{dfs, Graph, scc};
fn main() {
    let mut graph = Graph::new();
    graph.insert_edge(0, 4);
    graph.insert_edge(1, 0);
    graph.insert_edge(2, 1);
    graph.insert_edge(2, 3);
    graph.insert_edge(3, 2);
    graph.insert_edge(4, 1);
    graph.insert_edge(5, 4);
    graph.insert_edge(5, 1);
    graph.insert_edge(5, 6);
    graph.insert_edge(6, 5);
    graph.insert_edge(6, 2);
    graph.insert_edge(7, 6);
    graph.insert_edge(7, 3);
    graph.insert_edge(7, 7);
    let result = dfs(&graph, None);
    let scc_result = scc(&graph);
    println!("Graph is{} a dag\n", if result.dag { "" } else { " not" });
    println!("Topological sort:\n");
    for vertex in result.topological_sort {
        print!("{} ", vertex);
    }
    print!("\n");
    println!("Strongly connected components:\n");
    for i in 0..scc_result.sccs.len() {
        print!("{i}: ");
        for u in &scc_result.sccs[i] {
            print!("{}, ", *u);
        }
        print!("\n")
    }
}
