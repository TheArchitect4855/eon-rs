#[derive(Debug)]
pub struct EonError {
	pub(crate) kind: EonErrorKind,
	pub(crate) message: String,
}

#[derive(Debug)]
pub enum EonErrorKind {
	ParseError,
}