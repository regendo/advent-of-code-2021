use crate::helpers;

fn convert(c: char) -> Result<u8, Box<dyn std::error::Error>> {
	match c {
		'0'..='9' => Ok(c.to_string().parse::<u8>().unwrap()),
		_ => Err(format!("{} is not a digit.", c).into()),
	}
}

pub fn solve_1() {
	let heightmap = helpers::Grid::from(helpers::input_charwise! { u8, convert });
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
				local_minimums.push(curr)
			}
		}
	}

	let risk: u32 = local_minimums
		.iter()
		.map(|&height| u32::from(height) + 1)
		.sum();

	println!("Total risk level is {}.", risk);
}

pub fn solve_2() {
	todo!("part 2")
}
