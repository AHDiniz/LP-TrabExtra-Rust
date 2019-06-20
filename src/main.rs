// LP-TrabExtra-Rust: Solving the grouping problem with the leader algorithm implemented in Rust.
// 
// Alan Herculano Diniz
// 
// main.rs: program's entry point

mod leader;
mod reader;

use std::env;

fn main()
{
	let args: Vec<String> = env::args().collect();
	let length: usize = args.len();

	let e = String::from("entrada.txt");
	let d = String::from("distancia.txt");

	let mut pointsFile = if length == 1 { &e } else { &args[1] };
	let mut distFile = if length == 1 { &d } else { &args[2] };

	println!("{} {}", pointsFile, distFile);
}
