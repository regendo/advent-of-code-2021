use crate::helpers;
use itertools::Itertools;

pub fn solve_1() {
	let depth_measurements = helpers::input! {u64};
	let increases = depth_measurements
		.iter()
		.tuple_windows()
		.filter(|(a, b)| a < b)
		.count();
	println!("The measurements increased {} times.", increases)
}

pub fn solve_2() {
	let depth_measurements = helpers::input! {u64};
	let increases = depth_measurements
		.iter()
		.tuple_windows::<(_, _, _)>()
		.map(|(a, b, c)| a + b + c)
		.tuple_windows()
		.filter(|(a, b)| a < b)
		.count();
	println!(
		"The three-measurement sliding window sums increase {} times.",
		increases
	)
}
