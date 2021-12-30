use std::{collections::HashMap, fmt::Display};
use super::EonValue;

#[derive(Debug, PartialEq)]
pub struct EonObject(
	pub(crate) HashMap<String, EonValue>
);

impl EonObject {
	pub fn new() -> Self {
		EonObject(
			HashMap::new()
		)
	}

	pub fn set(&mut self, key: String, value: EonValue) -> Option<EonValue> {
		self.0.insert(key, value)
	}

	pub fn get(&self, key: &str) -> Option<&EonValue> {
		self.0.get(key)
	}

	pub fn remove(&mut self, key: &str) -> Option<EonValue> {
		self.0.remove(key)
	}
}

impl Display for EonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = Vec::with_capacity(self.0.len());
		for (key, value) in &self.0 {
			values.push(
				format!("{}: {}", key, value)
			);
		}

		let interior = values.join(", ");
		write!(f, "({})", interior)
    }
}