pub struct Grid<T>(Vec<Vec<T>>);

/// 2-dimensional grid of any type
impl<T> Grid<T> {
	pub fn bounding_box(&self) -> (usize, usize) {
		let height = self.0.len();
		let width = self.0.iter().map(|l| l.len()).max().expect("Empty!");
		(width, height)
	}

	pub fn get(&self, x: usize, y: usize) -> Option<&T> {
		self.0.get(y).and_then(|line| line.get(x))
	}

	pub fn surrounding_pos(&self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
		let bb = self.bounding_box();
		if self.get(x, y).is_some() {
			Some(
				[
					if x > 0 { Some((x - 1, y)) } else { None },
					if y > 0 { Some((x, y - 1)) } else { None },
					if x + 1 < bb.0 { Some((x + 1, y)) } else { None },
					if y + 1 < bb.1 { Some((x, y + 1)) } else { None },
				]
				.into_iter()
				.flatten()
				.collect(),
			)
		} else {
			None
		}
	}

	pub fn surrounding(&self, x: usize, y: usize) -> Option<Vec<&T>> {
		self.surrounding_pos(x, y).map(|positions| {
			positions
				.into_iter()
				.filter_map(|(x, y)| self.get(x, y))
				.collect()
		})
	}
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
	fn from(source: Vec<Vec<T>>) -> Self {
		Self(source)
	}
}
