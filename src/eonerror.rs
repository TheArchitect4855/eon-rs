use std::{fmt::Display, io};

#[derive(Debug)]
pub struct EonError {
	pub(crate) kind: EonErrorKind,
	pub(crate) message: String,
}

#[derive(Debug)]
pub enum EonErrorKind {
	ParseError,
	IoError(io::Error),
}

impl Display for EonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Display for EonErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let name = match self {
    		EonErrorKind::ParseError => "Parse Error",
			EonErrorKind::IoError(_) => "Io Error",
		};

		write!(f, "{}", name)
    }
}