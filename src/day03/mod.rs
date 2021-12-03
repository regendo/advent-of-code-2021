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

	let power_consumption: u32 =
		u32::try_from(epsilon_rate).unwrap() * u32::try_from(gamma_rate).unwrap();
	println!(
		"The submarine's power consumption is {}, and that's probably kilowatts?",
		power_consumption
	);
}

pub fn solve_2() {
	todo!()
}
