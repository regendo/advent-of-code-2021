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
				.map(|y| Point(a.0, y))
				.collect(),
			Line::Diagonal { from: a, to: b } => if a.1 <= b.1 { a.1..=b.1 } else { b.1..=a.1 }
				.map(|y| {
					// Diagonals are only at exactly 45°
					// Meaning the absolute distance |x_curr - x_0| is always equal to |y_curr - y_0|
					let abs_dist = if y <= a.1 { a.1 - y } else { y - a.1 };
					let to_the_right = a.0 <= b.0;
					let x = if to_the_right {
						a.0 + abs_dist
					} else {
						a.0 - abs_dist
					};
					Point(x, y)
				})
				.collect(),
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
