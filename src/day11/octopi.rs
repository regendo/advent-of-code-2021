#[derive(Debug, Copy, Clone)]
pub struct Octopus {
	energy: u8,
	flashed: bool,
}

impl Octopus {
	pub fn init_step(&mut self) {
		self.flashed = false;
	}

	pub fn gather_energy(&mut self) -> bool {
		if !self.flashed {
			if self.energy >= 9 {
				self.energy = 0;
				self.flashed = true;
				return true;
			} else {
				self.energy += 1;
			}
		}
		false
	}
}

impl TryFrom<char> for Octopus {
	type Error = String;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'0'..='9' => Ok(Octopus {
				energy: u8::try_from(value.to_digit(10).unwrap()).unwrap(),
				flashed: false,
			}),
			_ => Err(format!("Invalid character {}.", value)),
		}
	}
}
