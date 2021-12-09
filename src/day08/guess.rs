use core::hash::Hash;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Guess<T>
where
	T: Hash + PartialEq + Eq + Clone,
{
	Solved(T),
	Any(HashSet<T>),
}

impl<T> Guess<T>
where
	T: Hash + PartialEq + Eq + Clone,
{
	/// Try to convert this guess into a solution.
	/// If the guess was mutated this way, `true` is returned.
	pub fn solve(&mut self) -> bool {
		if let Guess::Any(guesses) = self {
			if guesses.len() == 1 {
				*self = Guess::Solved(guesses.iter().next().unwrap().to_owned());
				return true;
			}
		}
		false
	}

	/// Get the contained `Solved` value, or `None` if the guess is `Any`.
	pub fn get_solved(&self) -> Option<T> {
		if let Guess::Solved(inner) = self {
			Some(inner.clone())
		} else {
			None
		}
	}

	/// Get a mutable reference to the contained `Any` value, or `None` if the guess is `Solved`.
	pub fn get_any_mut(&mut self) -> Option<&mut HashSet<T>> {
		if let Guess::Any(inner) = self {
			Some(inner)
		} else {
			None
		}
	}
}
