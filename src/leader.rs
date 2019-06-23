// LP-TrabExtra-Rust: Solving the grouping problem with the leader algorithm implemented in Rust.
// 
// Alan Herculano Diniz
// 
// leader.rs: leader algorithm implementation

// Calculating the sse and the groups vector
//
// Inputs: the points vector and the limit distance
pub fn calculate_results(points: Vec<Vec<f64>>, limit: f64) -> (f64, Vec<Vec<usize>>)
{
	let mut groups: Vec<Vec<usize>> = vec![vec![]];
	groups[0].push(1);

	let length = points.len();
	let mut i: usize = 1;
	while i < length
	{
		let mut leader: bool = true;

		let g_length: usize = groups.len();
		let mut j: usize = 0;
		while j < g_length
		{
			if point_distance(&points[groups[j][0] - 1], &points[i]) <= limit
			{
				groups[j].push(i + 1);
				leader = false;
				break;
			}
			j += 1;
		}
		if leader
		{
			groups.push(vec![i + 1]);
		}

		i += 1;
	}

	let sse: f64 = calculate_sse(&points, &groups);

	return (sse, groups);
}

// Calculating the distance between two given points:
fn point_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64
{
	assert!(a.len() == b.len());
	let mut dist: f64 = 0.0;
	
	let mut i: usize = 0;
	while i < a.len()
	{
		let x: f64 = (a[i] - b[i]) * (a[i] - b[i]);
		dist += x;
		i += 1;
	}

	dist = dist.sqrt();
	
	return dist;
}

// Calculating the center of mass of a certain point group
// given the group indeces and the points vector:
fn center_of_mass(points: &Vec<Vec<f64>>, group: &Vec<usize>) -> Vec<f64>
{
	let mut center: Vec<f64> = vec![];

	let length = points[0].len();

	let mut i: usize = 0;
	while i < length
	{
		center.push(0.0);
		i += 1;
	}

	let devider: f64 = group.len() as f64;
	for index in group
	{
		let mut j: usize = 0;
		while j < length
		{
			center[j] += points[index - 1][j] / devider;
			j += 1;
		}
	}

	return center;
}

// Calculating the sse off a given group:
fn calculate_sse(points: &Vec<Vec<f64>>, groups: &Vec<Vec<usize>>) -> f64
{
	let mut sse: f64 = 0.0;

	for group in groups
	{
		let center: Vec<f64> = center_of_mass(points, group);
		
		for index in group
		{
			let dist: f64 = point_distance(&points[index - 1], &center);
			sse += dist * dist;
		}
	}

	return sse;
}
