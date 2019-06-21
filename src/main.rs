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

	let points_file = if length == 1 { &e } else { &args[1] };
	let dist_file = if length == 1 { &d } else { &args[2] };

	let (points, limit) = reader::parse_input(points_file, dist_file);
	let (sse, groups) = leader::calculate_results(points, limit);

	println!("sse = {}", sse);

	for point in groups
	{
		for num in point
		{
			print!("{} ", num);
		}
		print!("\n");
	}
}
