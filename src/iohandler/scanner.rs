//! Scanner
//!
//! Controls output and write operations. Utility abstraction over general stdio output/write operations.
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

/// Collect bytes from ANSI document and return io::Lines.
pub fn read_ansi<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
	P: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(BufReader::new(file).lines())
}

/// Collect io::Lines into list of strings.
pub fn parse_art(filename: &str, width: usize) -> Result<Vec<String>> {
	let mut linevec = Vec::new();

	read_ansi(filename).and_then(|lines| {
		for line in lines {
			let line = line?;
			if !line.starts_with("?>=") {
				linevec.push(
					String::from(line)
						.chars()
						.into_iter()
						.take(width)
						.collect::<String>() + "\n",
				);
			}
		}
		Ok(())
	})?;

	Ok(linevec)
}
