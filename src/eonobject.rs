use std::collections::HashMap;
use super::EonValue;

#[derive(Debug, PartialEq)]
pub struct EonObject(pub(crate) HashMap<String, EonValue>);

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