mod bracket;
mod syntax;

use crate::helpers;
use bracket::Bracket;

fn score(bracket: &Bracket) -> u32 {
	match bracket {
		Bracket::ClosingParenthesis => 3,
		Bracket::ClosingBracket => 57,
		Bracket::ClosingCurly => 1197,
		Bracket::ClosingArrow => 25137,
		_ => unimplemented!(),
	}
}

pub fn solve_1() {
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
	todo!()
}
