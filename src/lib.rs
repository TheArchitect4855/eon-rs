mod eonvalue;
use std::{path::Path, fs};

pub use eonvalue::EonValue;

mod eonobject;
pub use eonobject::EonObject;

mod eonerror;
pub use eonerror::*;

mod parser;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, EonError>;

pub fn load<P: AsRef<Path>>(path: P) -> Result<EonValue> {
	let source = match fs::read_to_string(path) {
		Ok(v) => v,
		Err(e) => return Err(EonError {
			message: e.to_string(),
			kind: EonErrorKind::IoError(e),
		}),
	};

	parse(&source)
}

pub fn parse(source: &str) -> Result<EonValue> {
	let source = source.trim();
	let chars: Vec<char> = source
		.chars()
		.collect();

	let res = parser::parse(source, &chars)?;
	if res.1 < source.len() {
		Err(EonError {
			kind: EonErrorKind::ParseError,
			message: format!("Unexpected data at end of file: {}", &source[res.1..]),
		})
	} else {
		Ok(res.0)
	}
}