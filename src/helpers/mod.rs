macro_rules! input_nums {
	() => {
		include_str!("input.txt")
			.lines()
			.map(|line| {
				line
					.parse::<u64>()
					.unwrap_or_else(|_| panic!("Unable to convert {} into a number.", line))
			})
			.collect()
	};
}
pub(crate) use input_nums;
