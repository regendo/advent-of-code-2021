use std::collections::HashSet;

/// Build a set of a string's unique characters.
pub fn set(s: &str) -> HashSet<char> {
	HashSet::from_iter(s.chars())
}
