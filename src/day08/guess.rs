use core::hash::Hash;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Guess<T>
where
	T: Hash + PartialEq + Eq + Clone,
{
	Solution(T),
	Any(HashSet<T>),
}

impl<T> Guess<T>
where
	T: Hash + PartialEq + Eq + Clone,
{
	/// Try to convert this guess into a solution.
	pub fn attempt(&mut self) {
		if let Guess::Any(guesses) = self {
			if guesses.len() == 1 {
				*self = Guess::Solution(guesses.iter().next().unwrap().to_owned());
			}
		}
	}
}
