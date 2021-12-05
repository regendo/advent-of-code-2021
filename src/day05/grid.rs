#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(u16, u16);

#[derive(Debug, Clone, Copy)]
pub enum Line {
	Horizontal { from: Point, to: Point },
	Vertical { from: Point, to: Point },
	Diagonal { from: Point, to: Point },
}

impl Line {
	pub fn points(&self) -> Vec<Point> {
		match self {
			Line::Horizontal { from: a, to: b } => if a.0 <= b.0 { a.0..=b.0 } else { b.0..=a.0 }
				.map(|x| Point(x, a.1))
				.collect(),
			Line::Vertical { from: a, to: b } => if a.1 <= b.1 { a.1..=b.1 } else { b.1..=a.1 }
				.map(|x| Point(a.0, x))
				.collect(),
			Line::Diagonal { from: a, to: b } => todo!(),
		}
	}
}

impl std::str::FromStr for Point {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.trim().split(',');
		Ok(Point(
			split.next().unwrap().parse().unwrap(),
			split.next().unwrap().parse().unwrap(),
		))
	}
}

impl std::str::FromStr for Line {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.trim().split("->");
		let from = split.next().unwrap().parse().unwrap();
		let to = split.next().unwrap().parse().unwrap();
		Ok(match (from, to) {
			(Point(a, _), Point(b, _)) if a == b => Line::Vertical { from, to },
			(Point(_, a), Point(_, b)) if a == b => Line::Horizontal { from, to },
			_ => Line::Diagonal { from, to },
		})
	}
}
