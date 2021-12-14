#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum Bracket {
	OpeningParenthesis,
	OpeningBracket,
	OpeningCurly,
	OpeningArrow,
	ClosingParenthesis,
	ClosingBracket,
	ClosingCurly,
	ClosingArrow,
}

impl Bracket {
	pub fn is_opening(&self) -> bool {
		use Bracket::*;
		matches!(
			self,
			OpeningArrow | OpeningCurly | OpeningBracket | OpeningParenthesis
		)
	}

	pub fn opposite(&self) -> Self {
		use Bracket::*;
		match self {
			OpeningParenthesis => ClosingParenthesis,
			OpeningBracket => ClosingBracket,
			OpeningCurly => ClosingCurly,
			OpeningArrow => ClosingArrow,
			ClosingParenthesis => OpeningParenthesis,
			ClosingBracket => OpeningBracket,
			ClosingCurly => OpeningCurly,
			ClosingArrow => OpeningArrow,
		}
	}

	pub fn matches(&self, other: &Self) -> bool {
		self.opposite() == *other
	}
}

impl TryFrom<char> for Bracket {
	type Error = String;

	fn try_from(s: char) -> Result<Self, Self::Error> {
		use Bracket::*;
		match s {
			'(' => Ok(OpeningParenthesis),
			'[' => Ok(OpeningBracket),
			'{' => Ok(OpeningCurly),
			'<' => Ok(OpeningArrow),
			')' => Ok(ClosingParenthesis),
			']' => Ok(ClosingBracket),
			'}' => Ok(ClosingCurly),
			'>' => Ok(ClosingArrow),
			_ => Err(format!("Invalid character {}.", s)),
		}
	}
}
