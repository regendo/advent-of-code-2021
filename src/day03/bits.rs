use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Bits {
	inner: Vec<bool>,
}

impl Bits {
	pub fn width(&self) -> usize {
		self.inner.len()
	}

	pub fn get(&self, pos: usize) -> Option<bool> {
		self.inner.get(pos).cloned()
	}

	pub fn set(&mut self, pos: usize, value: bool) {
		match self.inner.get_mut(pos) {
			Some(entry) => *entry = value,
			None => self.inner.resize(pos, value),
		}
	}

	pub fn push(&mut self, value: bool) {
		self.inner.push(value)
	}
}

impl FromStr for Bits {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let trimmed = s.trim();
		Ok(Bits {
			inner: trimmed
				.chars()
				.map(|c| match c {
					'0' => Ok(false),
					'1' => Ok(true),
					_ => Err(format!("Invalid character `{}`.", c)),
				})
				.collect::<Result<Vec<_>, _>>()?,
		})
	}
}

impl std::ops::Not for Bits {
	type Output = Bits;

	fn not(self) -> Self::Output {
		Bits {
			inner: self.into_iter().map(|b| !b).collect(),
		}
	}
}

impl IntoIterator for Bits {
	type Item = bool;
	type IntoIter = std::vec::IntoIter<bool>;

	fn into_iter(self) -> Self::IntoIter {
		self.inner.into_iter()
	}
}

impl TryFrom<Bits> for u32 {
	type Error = String;

	fn try_from(value: Bits) -> Result<Self, Self::Error> {
		u32::try_from(&value)
	}
}

impl TryFrom<&Bits> for u32 {
	type Error = String;

	fn try_from(value: &Bits) -> Result<Self, Self::Error> {
		if value.width() > 32 {
			Err("Doesn't fit into a u32.".to_owned())
		} else {
			Ok(value
				.clone()
				.into_iter()
				.map(|bit| match bit {
					false => 0,
					true => 1,
				})
				.fold(0, |number, digit| (number << 1) + digit))
		}
	}
}

impl From<&[bool]> for Bits {
	fn from(other: &[bool]) -> Self {
		Self {
			inner: other.to_vec(),
		}
	}
}

impl From<Vec<bool>> for Bits {
	fn from(other: Vec<bool>) -> Self {
		// Using From<&[bool]>
		Self::from(&*other)
	}
}

pub trait AverageAt {
	fn average_at(&self, pos: usize) -> bool;
}
impl AverageAt for Vec<Bits> {
	fn average_at(&self, pos: usize) -> bool {
		let (zeroes, ones) = self
			.iter()
			.map(|bits| bits.get(pos).or(Some(false)).unwrap())
			.fold((0, 0), |count, curr| match curr {
				false => (count.0 + 1, count.1),
				true => (count.0, count.1 + 1),
			});

		match zeroes.cmp(&ones) {
			std::cmp::Ordering::Less => true,
			std::cmp::Ordering::Equal => true,
			std::cmp::Ordering::Greater => false,
		}
	}
}
