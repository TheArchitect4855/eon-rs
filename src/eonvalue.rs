use std::fmt::Display;

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

impl Display for EonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let value = match self {
			EonValue::Bool(v) => v.to_string(),
			EonValue::Number(v) => v.to_string(),
			EonValue::Name(v) => v.to_string(),
			EonValue::String(v) => format!("'{}'", v),
			EonValue::Array(v) => {
				let values: Vec<String> = v.into_iter()
					.map(|v| {
						v.to_string()
					}).collect();
				
				let interior = values.join(", ");
				format!("[{}]", interior)
			},
			EonValue::Object(v) => v.to_string(),
			EonValue::Expression(v) => format!("{{{}}}", v),
		};

		write!(f, "{}", value)
    }
}