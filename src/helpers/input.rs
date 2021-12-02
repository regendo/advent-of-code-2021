/// Read input from this exercise's input.txt file and try to parse it into the provided type. The type must implement FromStr.
macro_rules! input {
	($target:ty) => {
		include_str!("input.txt")
			.lines()
			.map(|line| {
				line.parse::<$target>().unwrap_or_else(|e| {
					panic!(
						"Unable to parse `{}` into a {}: {:?}",
						line,
						::std::any::type_name::<$target>(),
						e
					)
				})
			})
			.collect::<Vec<$target>>()
	};
}
pub(crate) use input;
