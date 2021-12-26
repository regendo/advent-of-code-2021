use super::octopi;
use crate::helpers;

#[derive(Debug)]
struct Simulation(helpers::Grid<octopi::Octopus>);

impl Simulation {
	fn advance_one_step(&mut self) {
		todo!()
	}

	fn init_step(&mut self) {
		for oct in self.0.iter_mut() {
			oct.init_step()
		}
	}
}
