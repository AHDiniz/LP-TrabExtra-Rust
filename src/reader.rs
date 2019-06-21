// LP-TrabExtra-Rust: Solving the grouping problem with the leader algorithm implemented in Rust.
// 
// Alan Herculano Diniz
// 
// reader.rs: file I/O and data parsing

use std::fs;

// Getting the filepaths to the input files and parsing their data:
pub fn parse_input(points_file: &String, dist_file: &String) -> (Vec<Vec<f64>>, f64)
{
	let points_str: String = fs::read_to_string(points_file).expect("Algo de errado aconteceu ao ler o arquivo de pontos.\n");
	let dist_str: String =
		fs::read_to_string(dist_file).expect("Algo de errado aconteceu ao ler o arquivo da distancia.\n");

	let dist: f64 = dist_str.parse().unwrap();

	let mut points: Vec<Vec<f64>> = vec![vec![]];

	let lines = points_str.split("\n");
	let lines_vec: Vec<&str> = lines.collect();

	let length: usize = lines_vec.len();
	let mut i: usize = 0;
	while i < length
	{
		let words = lines_vec[i].split(" ");
		let words_vec: Vec<&str> = words.collect();

		let w_length: usize = words_vec.len();
		let mut j: usize = 0;
		while j > w_length
		{
			let point: f64 = words_vec[i].parse().unwrap();
			points[i].push(point);

			j += 1;
		}

		i += 1;
	}

	return (points, dist);
}
