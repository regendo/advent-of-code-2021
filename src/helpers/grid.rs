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

	pub fn surrounding(&self, x: usize, y: usize) -> Option<Vec<&T>> {
		if self.get(x, y).is_some() {
			Some(
				match (x, y) {
					(0, 0) => [None, None, Some((x + 1, y)), Some((x, y + 1))],
					(0, _) => [None, Some((x, y - 1)), Some((x + 1, y)), Some((x, y + 1))],
					(_, 0) => [Some((x - 1, y)), None, Some((x + 1, y)), Some((x, y + 1))],
					(_, _) => [
						Some((x - 1, y)),
						Some((x, y - 1)),
						Some((x + 1, y)),
						Some((x, y + 1)),
					],
				}
				.into_iter()
				.filter_map(|opt| opt.map(|(x, y)| self.get(x, y)))
				.flatten()
				.collect(),
			)
		} else {
			None
		}
	}
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
	fn from(source: Vec<Vec<T>>) -> Self {
		Self(source)
	}
}
