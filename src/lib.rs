mod eonvalue;
pub use eonvalue::EonValue;

mod eonobject;
pub use eonobject::EonObject;

mod eonerror;
pub use eonerror::*;

mod parser;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, EonError>;

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