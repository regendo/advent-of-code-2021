/// Read input from this exercise's input.txt file and try to parse each line into the provided type. The type must implement FromStr.
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

/// Read input from this exercise's input.txt file and try to parse each character into the provided type.
///
/// If you only provide the type, it must implement TryFrom<char>.
/// You can also provide a conversion function with a `|char| -> Result<u8, Box<dyn std::error::Error>>` signature.
macro_rules! input_charwise {
	($target:ty) => {
		include_str!("input.txt")
			.lines()
			.map(|line| {
				line
					.chars()
					.map(|c| {
						c.try_into().unwrap_or_else(|e| {
							panic!(
								"Unable to parse `{}` into a {}: {:?}",
								line,
								::std::any::type_name::<$target>(),
								e
							)
						})
					})
					.collect::<Vec<$target>>()
			})
			.collect::<Vec<_>>()
	};
	($target:ty, $convert:expr) => {
		include_str!("input.txt")
			.lines()
			.map(|line| {
				line
					.chars()
					.map(|c| {
						$convert(c).unwrap_or_else(|e: Box<dyn std::error::Error>| {
							panic!(
								"Unable to parse `{}` into a {}: {:?}",
								line,
								::std::any::type_name::<$target>(),
								e
							)
						})
					})
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>()
	};
}

pub(crate) use input;
pub(crate) use input_charwise;
