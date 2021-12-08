mod guess;
mod puzzle;
mod segment;

use itertools::Itertools;

use crate::helpers;

pub fn solve_1() {
	let puzzles = helpers::input! { puzzle::Puzzle };

	let digits = {
		use itertools::Itertools as _;

		puzzles
			.into_iter()
			.flat_map(|mut puzzle| {
				puzzle.easy_check();
				puzzle.answer()
			})
			.flatten()
			.counts()
	};
	let total: usize = digits.iter().map(|(_digit, count)| count).sum();

	println!("We've been able to decode these easy digits: {:?}", digits);
	println!("That's {} digits total.", total);
}

pub fn solve_2() {
	todo!()
}
