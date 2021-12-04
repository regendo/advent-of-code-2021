use std::str::FromStr;

mod bingo;
mod number_queue;

fn input() -> (number_queue::NumberQueue, Vec<bingo::BingoBoard>) {
	let mut lines = include_str!("input.txt").lines().peekable();
	let drawings = number_queue::NumberQueue::from_str(lines.next().unwrap()).unwrap();
	let mut boards = Vec::new();
	lines.next(); // Discard this empty line
	while lines.by_ref().peek().is_some() {
		let batch: [&str; 5] = lines
			.by_ref()
			.take(5)
			.collect::<Vec<_>>()
			.try_into()
			.unwrap();
		boards.push(bingo::BingoBoard::try_from(&batch).unwrap());

		lines.next(); // Discard this empty line
	}
	(drawings, boards)
}

pub fn solve_1() {
	let (drawings, mut boards) = input();
	for num in drawings {
		for board in &mut boards {
			board.mark(num);
			if board.is_bingo() {
				println!(
					"Bingo! First victory goes to final score {}.",
					board.score() * u32::from(num)
				);
				return;
			}
		}
	}
	println!("No bingo")
}

pub fn solve_2() {
	let (drawings, mut boards) = input();
	let mut score = None;

	for num in drawings {
		for board in &mut boards {
			if !board.is_bingo() {
				board.mark(num);
				if board.is_bingo() {
					score = Some(board.score() * u32::from(num));
				}
			}
		}
	}
	println!("Finally! The last bingo scored {}.", score.unwrap())
}
