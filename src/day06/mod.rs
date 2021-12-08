mod fish;

pub fn solve_1() {
	let mut swarm = include_str!("input.txt")
		.parse::<fish::ReproductionCycle>()
		.unwrap();

	for _ in 0..80 {
		swarm.advance();
	}

	println!("After 80 days, we've got {} fish.", swarm.count())
}

pub fn solve_2() {
	let mut swarm = include_str!("input.txt")
		.parse::<fish::ReproductionCycle>()
		.unwrap();

	for _ in 0..256 {
		swarm.advance();
	}

	println!("After 256 days, we've got {} fish.", swarm.count())
}
