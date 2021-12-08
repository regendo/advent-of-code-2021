use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

use super::guess::Guess;
use super::segment::{Digit, Segment};

#[derive(Debug)]
pub struct Puzzle {
	wires: HashMap<char, Guess<Segment>>,
	displays: HashMap<String, Guess<Digit>>,
	question: Vec<String>,
}

impl Puzzle {
	pub fn easy_check(&mut self) {
		for (display, digit) in &mut self.displays {
			if let Guess::Any(opts) = digit {
				// Eliminate all digits with wrong segment count
				opts
					.drain_filter(|d| d.segments().len() != display.chars().count())
					.for_each(drop);
				digit.attempt();
			}
		}
	}

	/// Try to answer the question asked as accurately as possible.
	/// This method doesn't attempt to solve the code, it only applies already found solutions to try to decode the question.
	pub fn answer(&self) -> Vec<Option<Digit>> {
		self
			.question
			.iter()
			.flat_map(|q| {
				self.displays.get(q).map(|g| {
					if let Guess::Solution(s) = g {
						Some(s.to_owned())
					} else {
						None
					}
				})
			})
			.collect()
	}
}

///
/// TRAIT IMPLS
///
impl FromStr for Puzzle {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split('|');
		let (tips, question) = (
			split
				.next()
				.unwrap()
				.split_whitespace()
				.map(|s| s.chars().sorted().collect::<String>())
				.collect::<Vec<_>>(),
			split
				.next()
				.unwrap()
				.split_whitespace()
				.map(|s| s.chars().sorted().collect::<String>())
				.collect(),
		);

		Ok(Self {
			wires: ('a'..='g')
				.map(|c| {
					(
						c,
						Guess::Any(HashSet::from_iter(Segment::digit(&Digit::Eight))),
					)
				})
				.collect(),
			displays: tips
				.into_iter()
				.map(|d| (d, Guess::Any(HashSet::from_iter(Digit::variants()))))
				.collect(),
			question,
		})
	}
}
