use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone)]
pub struct NumberQueue {
	inner: VecDeque<u8>,
}

impl Iterator for NumberQueue {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.pop_front()
	}
}

impl FromStr for NumberQueue {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			inner: s.split(',').map(|s| s.parse::<u8>().unwrap()).collect(),
		})
	}
}
