// LP-TrabExtra-Rust: Solving the grouping problem with the leader algorithm implemented in Rust.
// 
// Alan Herculano Diniz
// 
// reader.rs: file I/O and data parsing

use std::fs::File;
use std::io::{BufRead, BufReader};

// Getting the filepaths to the input files and parsing their data:
pub fn parse_input(points_file: &String, dist_file: &String) -> (Vec<Vec<f64>>, f64)
{
	// Reading the distance file:
	let d_file = File::open(dist_file).unwrap();
	let d_reader = BufReader::new(d_file);
	let mut dist: f64 = 0.0;
	for (_index, line) in d_reader.lines().enumerate()
	{
		let l: String = line.unwrap();
		dist = l.trim().parse().unwrap();
	}

	// Reading the points file:
	let p_file = File::open(points_file).unwrap();
	let p_reader = BufReader::new(p_file);
	let mut points: Vec<Vec<f64>> = vec![];
	for (_index, line) in p_reader.lines().enumerate()
	{
		let l: String = line.unwrap();
		let words: Vec<&str> = l.split(" ").collect();
		let length: usize = words.len();
		let mut point: Vec<f64> = vec![];
		let mut j: usize = 0;
		while j < length
		{
			let number: f64 = words[j].trim().parse().unwrap();
			point.push(number);
			j += 1;
		}
		points.push(point);
	}

	return (points, dist);
}
