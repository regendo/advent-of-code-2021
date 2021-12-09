use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

use super::guess::Guess;
use super::segment::{Digit, Segment};
use crate::helpers;

#[derive(Debug)]
pub struct Puzzle {
	wires: HashMap<char, Guess<Segment>>,
	displays: HashMap<String, Guess<Digit>>,
	question: Vec<String>,
}

impl Puzzle {
	/// Digits 1, 4, 7, and 8 have unique segment counts so we can immediately find them, even without knowing anything about how the wires are connected.
	pub fn easy_check(&mut self) {
		for (display, digit) in &mut self.displays {
			if let Guess::Any(opts) = digit {
				// Eliminate all digits with wrong segment count
				opts
					.drain_filter(|d| d.segments().len() != display.chars().count())
					.for_each(drop);
				digit.solve();
			}
		}
	}

	/// Sanitize our internal data.
	/// Among other things, cross-reference wires and displayed digits and double-check that individual [`Guess`es](enum@Guess) aren't solved yet.
	fn sanitize(&mut self) {
		while self.sanitize_wires() || self.sanitize_digits() {}
	}

	#[doc(hidden)]
	fn sanitize_wires(&mut self) -> bool {
		let mut modified = false;
		let (solved_wires, mut guessed_wires): (Vec<_>, Vec<_>) = self
			.wires
			.iter_mut()
			.partition(|(_, g)| matches!(g, Guess::Solved(_)));
		let solved_wires = solved_wires
			.iter()
			.map(|(_, g)| g.get_solved().unwrap())
			.collect::<Vec<_>>();

		for guess in guessed_wires.iter_mut() {
			// Any segments that are already solved are no longer valid options.
			// Also, cross-reference digits that match our key. Any segments they don't contain aren't valid options either.
			let potential_digits = self
				.displays
				.iter()
				.filter_map(|(k, v)| {
					if helpers::set(k).contains(guess.0) {
						Some(match v {
							Guess::Solved(digit) => digit.segments(),
							Guess::Any(digits) => digits.iter().flat_map(|d| d.segments()).collect(),
						})
					} else {
						None
					}
				})
				.reduce(|a, b| a.intersection(&b).copied().collect())
				.unwrap();

			guess
				.1
				.get_any_mut()
				.unwrap()
				.drain_filter(|s| solved_wires.contains(s) || !potential_digits.contains(s))
				.for_each(|_| modified = true);

			if guess.1.solve() {
				modified = true;
			}
		}

		modified
	}

	#[doc(hidden)]
	fn sanitize_digits(&mut self) -> bool {
		let mut modified = false;
		let (solved_digits, mut guessed_digits): (Vec<_>, Vec<_>) = self
			.displays
			.iter_mut()
			.partition(|(_, g)| matches!(g, Guess::Solved(_)));
		let solved_digits = solved_digits
			.iter()
			.map(|(_, g)| g.get_solved().unwrap())
			.collect::<Vec<_>>();

		for guess in guessed_digits.iter_mut() {
			// Any digits that are already solved are no longer valid options.
			guess
				.1
				.get_any_mut()
				.unwrap()
				.drain_filter(|d| solved_digits.contains(d))
				.for_each(|_| modified = true);
			if guess.1.solve() {
				modified = true;
			}
		}

		for digit in Digit::variants() {
			if !self.displays.values().contains(&Guess::Solved(digit))
				&& digit
					.segments()
					.iter()
					.all(|&s| self.wires.values().contains(&Guess::Solved(s)))
			{
				// This digit isn't marked as solved, but all of its segments are.
				// So we just need to look up how these segments are encoded
				let wires = helpers::set(
					&self
						.wires
						.iter()
						.filter_map(|(k, v)| {
							if matches!(v, Guess::Solved(_)) {
								Some((v.get_solved().unwrap(), k))
							} else {
								None
							}
						})
						.filter_map(|(v, k)| {
							if digit.segments().contains(&v) {
								Some(k)
							} else {
								None
							}
						})
						.collect::<String>(),
				);
				// and then we can find our digit
				*self
					.displays
					.iter_mut()
					.find(|(k, _)| helpers::set(k) == wires)
					.unwrap()
					.1 = Guess::Solved(digit);
				modified = true;
			}
		}
		modified
	}

	/// Helper function to reverse lookup an already solved digit.
	fn by_digit(&self, digit: Digit) -> Option<HashSet<char>> {
		self.displays.iter().find_map(|(k, v)| match v {
			Guess::Solved(value) if *value == digit => Some(helpers::set(k)),
			_ => None,
		})
	}

	/// Find all digits and wires.
	pub fn advanced_check(&mut self) {
		// Solves digits 1, 4, 7, 8
		self.easy_check();

		let one = self.by_digit(Digit::One).unwrap();
		let _four = self.by_digit(Digit::Four).unwrap();
		let seven = self.by_digit(Digit::Seven).unwrap();
		let eight = self.by_digit(Digit::Eight).unwrap();

		// Segment TopFlat
		{
			let top_flat = seven.difference(&one).next().unwrap();
			*self.wires.get_mut(top_flat).unwrap() = Guess::Solved(Segment::TopFlat);
			for wire in &one {
				*self.wires.get_mut(wire).unwrap() = Guess::Any(Digit::One.segments());
			}
		}

		// Digit 6
		{
			let six = self
				.displays
				.iter_mut()
				.map(|(k, v)| (helpers::set(k), v))
				.filter(|(s, _)| eight.difference(s).count() == 1)
				// 0, 6, or 9 have one less segment than 8
				// and since we already know 1, we can find 6 which negatively overlaps with it
				.find(|(s, _)| one.difference(s).count() == 1)
				.unwrap();
			*six.1 = Guess::Solved(Digit::Six);
		}

		let six = self.by_digit(Digit::Six).unwrap();
		// Segment TopRight
		// Digit 2
		{
			let top_right = one.difference(&six).next().unwrap();
			*self.wires.get_mut(top_right).unwrap() = Guess::Solved(Segment::TopRight);
			let bottom_right = one.iter().find(|c| *c != top_right).unwrap();
			*self.wires.get_mut(bottom_right).unwrap() = Guess::Solved(Segment::BottomRight);

			let two = self
				.displays
				.iter_mut()
				.find(|(k, _)| k.contains(*top_right) && !k.contains(*bottom_right))
				.unwrap();
			*two.1 = Guess::Solved(Digit::Two);
		}

		// At this point, the program can figure the rest out.
		self.sanitize();
	}

	/// Try to answer the question asked as accurately as possible.
	/// This method doesn't attempt to solve the code, it only applies already found solutions to try to decode the question.
	pub fn answer(&self) -> Vec<Option<Digit>> {
		self
			.question
			.iter()
			.flat_map(|q| {
				self.displays.get(q).map(|g| {
					if let Guess::Solved(s) = g {
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
