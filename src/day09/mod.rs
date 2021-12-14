use std::collections::VecDeque;

use crate::helpers;

fn convert(c: char) -> Result<u8, Box<dyn std::error::Error>> {
	match c {
		'0'..='9' => Ok(c.to_string().parse::<u8>().unwrap()),
		_ => Err(format!("{} is not a digit.", c).into()),
	}
}

fn get_local_minimums(heightmap: &helpers::Grid<u8>) -> Vec<(usize, usize)> {
	let cave_dimensions = heightmap.bounding_box();
	let mut local_minimums = vec![];

	for x in 0..cave_dimensions.0 {
		for y in 0..cave_dimensions.1 {
			let curr = *heightmap.get(x, y).unwrap();
			if heightmap
				.surrounding(x, y)
				.unwrap()
				.iter()
				.all(|&&height| height > curr)
			{
				local_minimums.push((x, y))
			}
		}
	}

	local_minimums
}

pub fn solve_1() {
	let heightmap = helpers::Grid::from(helpers::input_charwise! { u8, convert });
	let risk: u32 = get_local_minimums(&heightmap)
		.iter()
		.map(|(x, y)| heightmap.get(*x, *y).unwrap())
		.map(|&height| u32::from(height) + 1)
		.sum();

	println!("Total risk level is {}.", risk);
}

pub fn solve_2() {
	let heightmap = helpers::Grid::from(helpers::input_charwise! {u8, convert });
	let local_minimums = get_local_minimums(&heightmap);
	let basins = local_minimums.iter().map(|min| {
		// Expand outward from each low point until we've mapped its basin.
		// That being all points under height 9 that flow into this low point.
		let mut basin = vec![(min.0, min.1)];
		let mut previously_considered = vec![(min.0, min.1)];
		let surrounding = heightmap.surrounding_pos(min.0, min.1).unwrap();
		let mut queued = surrounding
			.into_iter()
			.filter(|(x, y)| heightmap.get(*x, *y).unwrap() < &9)
			.collect::<VecDeque<_>>();

		while !queued.is_empty() {
			let (x, y) = queued.pop_front().unwrap();
			let height_here = heightmap.get(x, y).unwrap();
			previously_considered.push((x, y));

			let surrounding = heightmap.surrounding_pos(x, y).unwrap();
			for (x, y) in surrounding.iter() {
				let height = heightmap.get(*x, *y).unwrap();
				if height < &9
					&& height > height_here
					&& !previously_considered.contains(&(*x, *y))
					&& !queued.contains(&(*x, *y))
				{
					basin.push((*x, *y));
					queued.push_back((*x, *y));
				}
			}
		}

		basin
	});

	use itertools::Itertools as _;
	let res = basins
		.into_iter()
		.map(|basin| basin.len())
		.sorted()
		.rev()
		.map(|len| {
			dbg!(&len);
			len
		})
		.take(3)
		.reduce(|a, b| a * b)
		.unwrap();

	println!("The three largest basins multiply to {}.", res);
}
