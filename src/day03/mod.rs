use crate::helpers;
mod bits;
use bits::Bits;

pub fn solve_1() {
	let diagnostic_report = helpers::input! { Bits };
	let gamma_rate = Bits::from(
		// Obviously breaks if the report is empty
		// Can give wrong result if reports have different widths
		// But neither will happen during the exercise
		(0..diagnostic_report.get(0).unwrap().width())
			.map(|pos| {
				use bits::AverageAt as _;
				diagnostic_report.average_at(pos)
			})
			.collect::<Vec<_>>(),
	);
	let epsilon_rate = !gamma_rate.clone();

	let power_consumption =
		u32::try_from(epsilon_rate).unwrap() * u32::try_from(gamma_rate).unwrap();
	println!(
		"The submarine's power consumption is {}, and that's probably kilowatts?",
		power_consumption
	);
}

pub fn solve_2() {
	let diagnostic_report = helpers::input! { Bits };

	let mut oxygen_generator_rating = diagnostic_report.clone();
	let mut co2_scrubber_rating = diagnostic_report.clone();
	for pos in 0..diagnostic_report.get(0).unwrap().width() {
		if oxygen_generator_rating.len() > 1 {
			let most_common = (0..oxygen_generator_rating.get(0).unwrap().width())
				.map(|pos| {
					use bits::AverageAt as _;
					oxygen_generator_rating.average_at(pos)
				})
				.collect::<Vec<_>>();
			oxygen_generator_rating = oxygen_generator_rating
				.into_iter()
				.filter(|report| report.get(pos).unwrap() == *most_common.get(pos).unwrap())
				.collect()
		}
		if co2_scrubber_rating.len() > 1 {
			let most_common = (0..co2_scrubber_rating.get(0).unwrap().width())
				.map(|pos| {
					use bits::AverageAt as _;
					co2_scrubber_rating.average_at(pos)
				})
				.collect::<Vec<_>>();
			co2_scrubber_rating = co2_scrubber_rating
				.into_iter()
				.filter(|report| report.get(pos).unwrap() != *most_common.get(pos).unwrap())
				.collect()
		}
	}

	let life_support_rating = u32::try_from(oxygen_generator_rating.get(0).unwrap()).unwrap()
		* u32::try_from(co2_scrubber_rating.get(0).unwrap()).unwrap();
	println!("Our life support rating is {}.", life_support_rating)
}
