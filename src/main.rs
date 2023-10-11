mod graph;
use crate::graph::{Graph, dfs};
fn main() {
    let mut graph = Graph::new();
    graph.insert_edge(0, 1);
    graph.insert_edge(0, 6);
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
    println!("Topological sort:\n");
    for vertex in result.topological_sort {
        print!("{} ", vertex);
    }
    print!("\n");
}
