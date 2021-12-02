use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::helpers;

#[derive(Debug, Clone, Copy, Default)]
pub struct SimplePosition {
	pub horizontal: u32,
	pub depth: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ComplexPosition {
	pub horizontal: i32,
	pub depth: i32,
	pub aim: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
	Forward(u32),
	Up(u32),
	Down(u32),
}

pub static COMMAND: Lazy<Regex> =
	helpers::simple_regex!(r"^(?P<direction>forward|up|down)\s(?P<distance>\d+)$");

impl FromStr for Command {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(cap) = COMMAND.captures(s) {
			let direction = cap.name("direction").unwrap().as_str();
			let distance: u32 = cap.name("distance").unwrap().as_str().parse().unwrap();
			match &*direction.to_lowercase() {
				"forward" => Ok(Command::Forward(distance)),
				"up" => Ok(Command::Up(distance)),
				"down" => Ok(Command::Down(distance)),
				_ => Err(format!("Unknown command {}.", direction)),
			}
		} else {
			Err(format!("Can't construct a command from `{}`.", s))
		}
	}
}

impl Command {
	pub fn apply_v1(self, pos: SimplePosition) -> SimplePosition {
		match self {
			Command::Forward(dist) => SimplePosition {
				horizontal: pos.horizontal + dist,
				depth: pos.depth,
			},
			Command::Up(dist) => SimplePosition {
				horizontal: pos.horizontal,
				depth: pos.depth - dist,
			},
			Command::Down(dist) => SimplePosition {
				horizontal: pos.horizontal,
				depth: pos.depth + dist,
			},
		}
	}

	pub fn apply_v2(self, pos: ComplexPosition) -> ComplexPosition {
		match self {
			Command::Forward(dist) => ComplexPosition {
				horizontal: pos.horizontal + i32::try_from(dist).unwrap(),
				depth: pos.depth + i32::try_from(dist).unwrap() * pos.aim,
				aim: pos.aim,
			},
			Command::Up(angle) => ComplexPosition {
				horizontal: pos.horizontal,
				depth: pos.depth,
				aim: pos.aim - i32::try_from(angle).unwrap(),
			},
			Command::Down(angle) => ComplexPosition {
				horizontal: pos.horizontal,
				depth: pos.depth,
				aim: pos.aim + i32::try_from(angle).unwrap(),
			},
		}
	}
}
