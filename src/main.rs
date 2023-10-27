use grph::{Graph, Component, State};
use std::{env, collections::HashMap};
use execute::{Execute, shell};
use std::process::Stdio;

fn make<'a>(g: &'a Graph, c: &'a Component, st: &mut HashMap<&'a Component, State>) {
    st.insert(c, State::Visited);
    for dep in &c.dependencies {
        let cdep = g.make.get(dep).expect("Validation error");
        match *st.get(cdep).expect("No state") {
            State::Visited => panic!("Cant have cycle in dependencies"),
            State::Unexplored => make(g, cdep, st),
            State::Finished => ()
        }
    }
    st.insert(c, State::Finished);
    println!("{}", c.run);
    let mut command = shell(&c.run[..]);
    command.stdout(Stdio::piped());
    let output = command.execute_output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}
fn main() {
    let target: Vec<String> = env::args().skip(1).collect();
    match target.len() {
        1 => {
            let target = &target[0];
            println!("{:?}", target);
            let g = Graph::new()
                .read_from_file("deps.yaml")
                .expect("A target depends on non existing target");
            let c = g.make.get(target).expect("Target non existent");
            let mut state_tracker = HashMap::new();
            for (_, c) in &g.make {
                state_tracker.insert(c, State::Unexplored);
            }
            make(&g, c, &mut state_tracker);
        }
        _ => ()
    }
}
