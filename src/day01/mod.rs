use crate::helpers;
use itertools::Itertools;

pub fn solve_1() {
	let depth_measurements: Vec<_> = helpers::input_nums!();
	let increases = depth_measurements
		.iter()
		.tuple_windows()
		.filter(|(a, b)| a < b)
		.count();
	println!("The measurements increased {} times.", increases)
}
