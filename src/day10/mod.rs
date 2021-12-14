mod bracket;
mod syntax;

use crate::helpers;
use bracket::Bracket;

pub fn solve_1() {
	fn score(bracket: &Bracket) -> u32 {
		match bracket {
			Bracket::ClosingParenthesis => 3,
			Bracket::ClosingBracket => 57,
			Bracket::ClosingCurly => 1197,
			Bracket::ClosingArrow => 25137,
			_ => unimplemented!(),
		}
	}

	let lines = helpers::input_charwise! { Bracket };
	let score: u32 = lines
		.into_iter()
		.map(|line| syntax::validate(&line))
		.filter_map(|res| res.err())
		.filter_map(|err| match err {
			syntax::Error::Corrupted { found, .. } => Some(score(&found)),
			_ => None,
		})
		.sum();
	println!("Our total syntax error score is {}.", score);
}

pub fn solve_2() {
	fn char_score(bracket: &Bracket) -> u32 {
		match bracket {
			Bracket::ClosingParenthesis => 1,
			Bracket::ClosingBracket => 2,
			Bracket::ClosingCurly => 3,
			Bracket::ClosingArrow => 4,
			_ => unimplemented!(),
		}
	}
	fn line_score(line: &[Bracket]) -> u64 {
		line
			.iter()
			.map(char_score)
			.fold(0, |acc, curr| 5 * acc + u64::from(curr))
	}

	let lines = helpers::input_charwise! { Bracket };

	// for .sorted()
	use itertools::Itertools as _;
	let scores = lines
		.into_iter()
		.map(|line| syntax::validate(&line))
		.filter_map(|res| res.err())
		.filter_map(|err| match err {
			syntax::Error::Incomplete { missing } => Some(missing),
			_ => None,
		})
		.map(|autocomplete| line_score(&autocomplete))
		.sorted()
		.collect::<Vec<_>>();
	let middle = scores.get(scores.len() / 2).unwrap();

	println!("The middle autocompletion score is {}.", middle);
}
