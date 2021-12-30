use super::EonObject;

#[derive(Debug, PartialEq)]
pub enum EonValue {
	Bool(bool),
	Number(f64),
	Name(String),
	String(String),
	Array(Box<[EonValue]>),
	Object(Box<EonObject>),
	Expression(String),
}