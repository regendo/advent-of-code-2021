mod command;

use crate::helpers;
use command::Command;

pub fn solve_1() {
	let commands = helpers::input! { Command };
	let destination = commands
		.iter()
		.fold(command::SimplePosition::default(), |pos, curr| {
			curr.apply_v1(pos)
		});

	println!(
		"The secret number is {} (at {:?}).",
		destination.horizontal * destination.depth,
		destination
	)
}

pub fn solve_2() {
	let commands = helpers::input! { Command };
	let destination = commands
		.iter()
		.fold(command::ComplexPosition::default(), |pos, curr| {
			curr.apply_v2(pos)
		});

	println!(
		"The secret number is {} (at {:?}).",
		destination.horizontal * destination.depth,
		destination
	)
}
