use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::helpers;

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
	pub horizontal: u32,
	pub depth: u32,
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
	pub fn apply(self, pos: Position) -> Position {
		match self {
			Command::Forward(dist) => Position {
				horizontal: pos.horizontal + dist,
				depth: pos.depth,
			},
			Command::Up(dist) => Position {
				horizontal: pos.horizontal,
				depth: pos.depth - dist,
			},
			Command::Down(dist) => Position {
				horizontal: pos.horizontal,
				depth: pos.depth + dist,
			},
		}
	}
}
