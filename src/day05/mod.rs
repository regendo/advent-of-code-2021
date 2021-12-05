use itertools::Itertools;

use crate::helpers;
use std::collections::HashMap;

mod grid;

fn count_occurrances(points: &[grid::Point]) -> HashMap<grid::Point, u32> {
	points.iter().fold(HashMap::new(), |mut count, curr| {
		*count.entry(*curr).or_insert(0) += 1;
		count
	})
}

pub fn solve_1() {
	let hydrothermal_vents = helpers::input! { grid::Line };
	let overlaps = count_occurrances(
		&hydrothermal_vents
			.iter()
			.filter(|line| !matches!(line, grid::Line::Diagonal { .. }))
			.flat_map(|line| line.points())
			.collect_vec(),
	)
	.iter()
	.filter(|(_, &count)| count >= 2)
	.count();

	println!(
		"The hydrothermal vents overlap at {} unique points (excluding diagonals).",
		overlaps
	);
}

pub fn solve_2() {
	let hydrothermal_vents = helpers::input! { grid::Line };
	let overlaps = count_occurrances(
		&hydrothermal_vents
			.iter()
			.flat_map(|line| line.points())
			.collect_vec(),
	)
	.iter()
	.filter(|(_, &count)| count >= 2)
	.count();

	println!(
		"The hydrothermal vents overlap at {} unique points (including diagonals).",
		overlaps
	);
}
