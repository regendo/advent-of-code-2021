type Fleet = Vec<usize>;
fn crabs() -> Fleet {
	use itertools::Itertools as _;

	include_str!("input.txt")
		.trim()
		.split(',')
		.map(|s| s.parse().unwrap())
		.sorted()
		.collect()
}

fn calc_optimal_position(fleet: Fleet, fuel_consumption: fn(usize) -> usize) -> (usize, usize) {
	let mut minimum: Option<(usize, usize)> = None;
	for pos in *fleet.first().unwrap()..=*fleet.last().unwrap() {
		let total_fuel_for_position = fleet
			.iter()
			.map(|&crab| fuel_consumption(if crab > pos { crab - pos } else { pos - crab }))
			.sum();
		if let Some((_, min)) = minimum {
			if min > total_fuel_for_position {
				minimum = Some((pos, total_fuel_for_position))
			}
		} else {
			minimum = Some((pos, total_fuel_for_position))
		}
	}
	minimum.unwrap()
}

pub fn solve_1() {
	let fuel_consumption = |dist| dist;
	let (position, fuel) = calc_optimal_position(crabs(), fuel_consumption);
	println!(
		"Best position with normal fuel consumption is at coordinate {}, reachable with a total {} fuel.",
		position, fuel
	)
}

pub fn solve_2() {
	let fuel_consumption = |n| n * (n + 1) / 2;
	let (position, fuel) = calc_optimal_position(crabs(), fuel_consumption);
	println!(
		"Best position with increasing fuel consumption is at coordinate {}, reachable with a total {} fuel.",
		position, fuel
	)
}
