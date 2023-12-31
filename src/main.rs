mod graph;
use crate::graph::read_from_file;
fn main() -> std::io::Result<()> {
    let mut graph = read_from_file("graph.yaml");
    println!("{:?}", graph);
    graph.insert_edge(6, 7);
    println!("{:?}", graph);
    graph.save_to_file("graph.yaml");
    Ok(())
}
