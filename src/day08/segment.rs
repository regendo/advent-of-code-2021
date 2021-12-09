use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Segment {
	TopFlat,
	TopLeft,
	TopRight,
	CenterFlat,
	BottomLeft,
	BottomRight,
	BottomFlat,
}

impl Segment {
	pub fn digit(digit: &Digit) -> HashSet<Segment> {
		use Digit::*;
		use Segment::*;

		HashSet::from_iter(match digit {
			Zero => vec![
				TopFlat,
				TopLeft,
				TopRight,
				BottomLeft,
				BottomRight,
				BottomFlat,
			],
			One => vec![TopRight, BottomRight],
			Two => vec![TopFlat, TopRight, CenterFlat, BottomLeft, BottomFlat],
			Three => vec![TopFlat, TopRight, CenterFlat, BottomRight, BottomFlat],
			Four => vec![TopLeft, TopRight, CenterFlat, BottomRight],
			Five => vec![TopFlat, TopLeft, CenterFlat, BottomRight, BottomFlat],
			Six => vec![
				TopFlat,
				TopLeft,
				CenterFlat,
				BottomLeft,
				BottomRight,
				BottomFlat,
			],
			Seven => vec![TopFlat, TopRight, BottomRight],
			Eight => vec![
				TopLeft,
				TopFlat,
				TopRight,
				CenterFlat,
				BottomLeft,
				BottomRight,
				BottomFlat,
			],
			Nine => vec![
				TopLeft,
				TopFlat,
				TopRight,
				CenterFlat,
				BottomRight,
				BottomFlat,
			],
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Digit {
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
}

impl Digit {
	pub fn variants() -> Vec<Self> {
		use Digit::*;
		vec![Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine]
	}

	pub fn segments(&self) -> HashSet<Segment> {
		Segment::digit(self)
	}
}

impl TryFrom<HashSet<Segment>> for Digit {
	type Error = String;

	fn try_from(value: HashSet<Segment>) -> Result<Self, Self::Error> {
		for digit in Digit::variants() {
			if digit.segments() == value {
				return Ok(digit);
			}
		}
		Err(format!("No digit matches the segments {:?}.", value))
	}
}

impl From<Digit> for u8 {
	fn from(d: Digit) -> Self {
		match d {
			Digit::Zero => 0,
			Digit::One => 1,
			Digit::Two => 2,
			Digit::Three => 3,
			Digit::Four => 4,
			Digit::Five => 5,
			Digit::Six => 6,
			Digit::Seven => 7,
			Digit::Eight => 8,
			Digit::Nine => 9,
		}
	}
}

impl From<Digit> for u32 {
	fn from(d: Digit) -> Self {
		u8::from(d).into()
	}
}
