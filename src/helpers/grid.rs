use std::slice::IterMut;

#[derive(Debug)]
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

	pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
		self.0.get_mut(y).and_then(|line| line.get_mut(x))
	}

	fn inner_surrounding_pos(
		&self,
		x: usize,
		y: usize,
		include_diagonals: bool,
	) -> Option<Vec<(usize, usize)>> {
		let bb = self.bounding_box();
		if self.get(x, y).is_some() {
			let ix = isize::try_from(x).unwrap();
			let iy = isize::try_from(y).unwrap();
			Some(
				if include_diagonals {
					vec![
						Some((ix - 1, iy)),
						Some((ix - 1, iy - 1)),
						Some((ix, iy - 1)),
						Some((ix + 1, iy - 1)),
						Some((ix + 1, iy)),
						Some((ix + 1, iy + 1)),
						Some((ix, iy + 1)),
						Some((ix - 1, iy + 1)),
					]
				} else {
					vec![
						Some((ix - 1, iy)),
						Some((ix, iy - 1)),
						Some((ix + 1, iy)),
						Some((ix, iy + 1)),
					]
				}
				.into_iter()
				.flatten()
				.filter_map(|(a, b)| {
					if a > 0
						&& b > 0 && usize::try_from(a).unwrap() < bb.0
						&& usize::try_from(b).unwrap() < bb.1
					{
						Some((usize::try_from(a).unwrap(), usize::try_from(b).unwrap()))
					} else {
						None
					}
				})
				.collect(),
			)
		} else {
			None
		}
	}

	pub fn surrounding_pos(&self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
		self.inner_surrounding_pos(x, y, false)
	}

	pub fn surrounding_pos_with_diagonals(&self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
		self.inner_surrounding_pos(x, y, true)
	}

	pub fn surrounding(&self, x: usize, y: usize) -> Option<Vec<&T>> {
		self.inner_surrounding_pos(x, y, false).map(|positions| {
			positions
				.into_iter()
				.filter_map(|(x, y)| self.get(x, y))
				.collect()
		})
	}

	pub fn surrounding_with_diagonals(&self, x: usize, y: usize) -> Option<Vec<&T>> {
		self.inner_surrounding_pos(x, y, true).map(|positions| {
			positions
				.into_iter()
				.filter_map(|(x, y)| self.get(x, y))
				.collect()
		})
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.0.iter_mut().flat_map(|inner| inner.iter_mut())
	}
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
	fn from(source: Vec<Vec<T>>) -> Self {
		Self(source)
	}
}
