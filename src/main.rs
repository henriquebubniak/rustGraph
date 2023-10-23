use grph::Graph;
use std::env;
fn main() {
    let target: Vec<String> = env::args().skip(1).collect();
    let target = &target[0];
    println!("{:?}", target);
    let g = Graph::new()
        .read_from_file("deps.yaml")
        .expect("A target depends on non existing target");
    let mut target_tree = Graph::new();
    match g.make.get(target) {
        Some(s) => (),
        None => (),
    }
    g.save_to_file("deps.yaml");
}
