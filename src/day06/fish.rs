use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReproductionCycle(HashMap<u8, usize>);

impl ReproductionCycle {
	pub fn advance(&mut self) {
		let days_of_rest = 6;
		let new_fish_start_at = 8;

		let todays_lucky_fishes = self.0.remove_entry(&0).unwrap_or((0, 0)).1;
		self.0 = self.0.iter().map(|(day, &fish)| (day - 1, fish)).collect();
		*self.0.entry(days_of_rest).or_insert(0) += todays_lucky_fishes;
		*self.0.entry(new_fish_start_at).or_insert(0) += todays_lucky_fishes;
	}

	pub fn count(&self) -> usize {
		self.0.values().sum()
	}
}

///
/// TRAIT IMPLS
///

impl From<HashMap<u8, usize>> for ReproductionCycle {
	fn from(other: HashMap<u8, usize>) -> Self {
		Self(other)
	}
}

impl std::str::FromStr for ReproductionCycle {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use itertools::Itertools as _;

		Ok(s
			.trim()
			.split(',')
			.map(|s| s.parse().unwrap())
			.counts()
			.into())
	}
}

///
/// TESTS
///
#[cfg(test)]
mod tests {
	use super::*;

	fn normalize(cycle: &ReproductionCycle) -> ReproductionCycle {
		cycle
			.0
			.clone()
			.into_iter()
			.filter(|(_, v)| v > &0)
			.collect::<HashMap<_, _>>()
			.into()
	}

	#[test]
	fn advances_correctly() -> Result<(), Box<dyn std::error::Error>> {
		let after_each_day: Vec<ReproductionCycle> = vec![
			"2,3,2,0,1".parse()?,
			"1,2,1,6,0,8".parse()?,
			"0,1,0,5,6,7,8".parse()?,
			"6,0,6,4,5,6,7,8,8".parse()?,
			"5,6,5,3,4,5,6,7,7,8".parse()?,
			"4,5,4,2,3,4,5,6,6,7".parse()?,
			"3,4,3,1,2,3,4,5,5,6".parse()?,
			"2,3,2,0,1,2,3,4,4,5".parse()?,
			"1,2,1,6,0,1,2,3,3,4,8".parse()?,
			"0,1,0,5,6,0,1,2,2,3,7,8".parse()?,
			"6,0,6,4,5,6,0,1,1,2,6,7,8,8,8".parse()?,
			"5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8".parse()?,
			"4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8".parse()?,
			"3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8".parse()?,
			"2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7".parse()?,
			"1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8".parse()?,
			"0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8".parse()?,
			"6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8".parse()?,
		];
		let mut current: ReproductionCycle = "3,4,3,1,2".parse()?;

		for expected in after_each_day {
			current.advance();
			assert_eq!(normalize(&current), expected);
		}

		Ok(())
	}

	#[test]
	fn counts_correctly() {
		let mut current: ReproductionCycle = "3,4,3,1,2".parse().unwrap();
		assert_eq!(current.count(), 5);

		for day in 1..=80 {
			current.advance();

			if day == 18 {
				assert_eq!(current.count(), 26);
			}
			if day == 80 {
				assert_eq!(current.count(), 5934);
			}
		}
	}
}
