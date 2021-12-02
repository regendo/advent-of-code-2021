mod command;

use crate::helpers;
use command::Command;

pub fn solve_1() {
	let commands = helpers::input! { Command };
	let destination = commands
		.iter()
		.fold(command::Position::default(), |pos, curr| curr.apply(pos));

	println!(
		"The secret number is {} (at {:?}).",
		destination.horizontal * destination.depth,
		destination
	)
}

pub fn solve_2() {
	todo!()
}
