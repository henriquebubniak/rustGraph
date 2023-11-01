use execute::{shell, Execute};
use grph::{Component, Graph};
use std::process::Stdio;
use std::env;

fn make<'a>(g: &'a Graph, c: &'a Component) {
    for dep in &c.dependencies {
        let cdep = g.make.get(dep).expect("All dependencies are expected to exist");
        make(g, cdep);
    }
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
            let g = match Graph::new().read_from_file("deps.yaml") {
                Ok(g) => g,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            let c = match g.make.get(target) {
                None => {
                    println!("Non-existing target");
                    return;
                }
                Some(c) => c
            };
            make(&g, c);
        }
        _ => (),
    }
}
