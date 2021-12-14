use super::bracket::Bracket;

#[derive(Debug)]
pub enum Error {
	Incomplete { missing: Vec<Bracket> },
	Corrupted { expected: Bracket, found: Bracket },
}

pub fn validate(line: &[Bracket]) -> Result<(), Error> {
	let mut nesting = Vec::new();
	for curr in line {
		if curr.is_opening() {
			nesting.push(curr);
		} else if nesting.last().expect("Empty!!").matches(curr) {
			nesting.pop();
		} else {
			return Err(Error::Corrupted {
				found: *curr,
				expected: nesting.last().expect("Empty???").opposite(),
			});
		}
	}

	if nesting.is_empty() {
		Ok(())
	} else {
		Err(Error::Incomplete {
			missing: nesting
				.into_iter()
				.rev()
				.map(|b| b.opposite())
				.collect::<Vec<_>>(),
		})
	}
}
