use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
enum Bingo {
	Guess(u8),
	Called(u8),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BingoBoard {
	inner: [[Bingo; 5]; 5],
}

impl BingoBoard {
	pub fn is_bingo(&self) -> bool {
		// I feel like this map-style check is a bit of an unholy abomination, but it'll work
		self
			.inner
			.iter()
			.any(|line| line.iter().all(|field| matches!(field, Bingo::Called(_))))
			|| (0..5).any(|col| {
				self
					.inner
					.iter()
					.all(|line| matches!(line[col], Bingo::Called(_)))
			})
	}

	pub fn score(&self) -> u32 {
		self
			.inner
			.iter()
			.flatten()
			.filter_map(|field| {
				if let Bingo::Guess(val) = field {
					Some(u32::from(*val))
				} else {
					None
				}
			})
			.reduce(|a, b| a + b)
			.unwrap()
	}

	pub fn mark(&mut self, num: u8) {
		self.inner.iter_mut().for_each(|line| {
			line.iter_mut().for_each(|field| {
				if matches!(field, Bingo::Guess(x) if *x == num) {
					*field = Bingo::Called(num);
				}
			})
		})
	}
}

impl<'arr, 'str> TryFrom<&'arr [&'str str; 5]> for BingoBoard {
	type Error = String;

	fn try_from(value: &[&str; 5]) -> Result<Self, Self::Error> {
		Ok(BingoBoard {
			inner: value.map(|line| {
				line
					.split_whitespace()
					.map(|s| Bingo::Guess(str::parse::<u8>(s).unwrap()))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap()
			}),
		})
	}
}
