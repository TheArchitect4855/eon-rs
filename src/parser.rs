use std::collections::HashMap;

use crate::{EonValue, EonError, EonErrorKind, EonObject};

use super::Result;

/*
	Data Types:
	Boolean
	Number
	Name
	String
	Expression
	Array
	Object
 */

pub fn parse(source: &str, chars: &[char]) -> Result<(EonValue, usize)> {
	let boolean = parse_bool(source, chars)?;
	let num = parse_number(source, chars)?;
	let name = parse_name(source, chars)?;
	let string = parse_string(source, chars)?;
	let expression = parse_expression(source, chars)?;
	let array = parse_array(source, chars)?;
	let object = parse_object(source, chars)?;
	if let Some(v) = boolean {
		Ok(v)
	} else if let Some(v) = num {
		Ok(v)
	} else if let Some(v) = name {
		Ok(v)
	} else if let Some(v) = string {
		Ok(v)
	} else if let Some(v) = expression {
		Ok(v)
	} else if let Some(v) = array {
		Ok(v)
	} else if let Some(v) = object {
		Ok(v)
	} else {
		Err(EonError {
			kind: EonErrorKind::ParseError,
			message: format!("Unexpected token near {}", &source[..5]),
		})
	}
}

pub fn parse_array(source: &str, chars: &[char]) -> Result<Option<(EonValue, usize)>> {
	let start = advance_whitespace(chars);
	let mut len = start;

	let first = chars.get(start);
	if first.is_none() || *first.unwrap() != '[' {
		return Ok(None);
	}

	len += 1;

	let mut values = Vec::new();
	loop {
		let value = parse(&source[len..], &chars[len..])?;
		values.push(value.0);
		len += value.1;

		len += advance_whitespace(&chars[len..]);

		let next = match chars.get(len) {
			Some(v) => *v,
			None => return Err(EonError {
				kind: EonErrorKind::ParseError,
				message: String::from("Unexpected EOF while parsing array (expected , or ])"),
			})
		};

		if next != ',' && next != ']' {
			return Err(EonError {
				kind: EonErrorKind::ParseError,
				message: format!("Unexpected token while parsing array (expected , or ], got '{}') :: {}", next, &source[len..]),
			});
		}

		if next == ',' {
			len += 1;

			let whitespace = advance_whitespace(&chars[len..]);
			let next = match chars.get(len + whitespace) {
				Some(v) => *v,
				None => return Err(EonError {
					kind: EonErrorKind::ParseError,
					message: String::from("Unexpected EOF while parsing array (expected ] or a value)"),
				})
			};

			if next == ']' {
				len += whitespace + 1;
				break;
			}
		} else if next == ']' {
			len += 1;
			break;
		}
	}

	Ok(Some((
		EonValue::Array(values.into_boxed_slice()),
		len
	)))
}

pub fn parse_bool(source: &str, chars: &[char]) -> Result<Option<(EonValue, usize)>> {
	let start = advance_whitespace(chars);
	let mut len = start;

	let literal = match_aplhanumeric(&source[len..], &chars[len..]);
	len += literal.len();

	if start == len {
		return Ok(None);
	}

	if literal == "true" {
		Ok(Some((
			EonValue::Bool(true),
			len,
		)))
	} else if literal == "false" {
		Ok(Some((
			EonValue::Bool(false),
			len,
		)))
	} else {
		Ok(None)
	}
}

pub fn parse_expression(source: &str, chars: &[char]) -> Result<Option<(EonValue, usize)>> {
	let start = advance_whitespace(chars);
	let mut len = start;

	let first = chars[start];
	if first != '{' {
		return Ok(None);
	}

	len += 1;

	let mut previous = first;
	for c in &chars[len..] {
		len += 1;

		if *c == '}' && previous != '\\' {
			previous = *c;
			break;
		}

		previous = *c;
	}

	if previous != '}' {
		return Err(EonError {
			kind: EonErrorKind::ParseError,
			message: String::from("Unexpected EOF while parsing expression"),
		})
	}

	Ok(Some((
		EonValue::Expression(
			String::from(&source[start + 1..len - 1])
		),
		len,
	)))
}

pub fn parse_number(source: &str, chars: &[char]) -> Result<Option<(EonValue, usize)>> {
	let start = advance_whitespace(chars);
	let mut len = start;

	let literal = match_number(&source[len..], &chars[len..]);
	len += literal.len();

	if start == len {
		return Ok(None);
	}

	let num = match literal.parse::<f64>() {
		Ok(v) => v,
		Err(e) => return Err(EonError {
			kind: EonErrorKind::ParseError,
			message: format!("Error parsing number: {}", e.to_string()),
		})
	};

	Ok(Some((
		EonValue::Number(num),
		len,
	)))
}

pub fn parse_name(source: &str, chars: &[char]) -> Result<Option<(EonValue, usize)>> {
	let start = advance_whitespace(chars);
	let mut len = start;

	let literal = match_name(&source[len..], &chars[len..]);
	len += literal.len();

	if start == len {
		return Ok(None);
	}

	Ok(Some((
		EonValue::Name(
			String::from(literal)
		),
		len,
	)))
}

pub fn parse_object(source: &str, chars: &[char]) -> Result<Option<(EonValue, usize)>> {
	let start = advance_whitespace(chars);
	let mut len = start;

	let first = chars.get(start);
	if first.is_none() || *first.unwrap() != '(' {
		return Ok(None);
	}

	len += 1;

	let mut map = HashMap::new();
	loop {
		len += advance_whitespace(&chars[len..]);
		
		let key = match_key(&source[len..], &chars[len..]);
		len += key.len();

		if key.len() == 0 {
			return Err(EonError{
				kind: EonErrorKind::ParseError,
				message: String::from("Invalid key in object"),
			});
		}

		len += advance_whitespace(&chars[len..]);

		let next = match chars.get(len) {
			Some(v) => *v,
			None => return Err(EonError {
				kind: EonErrorKind::ParseError,
				message: String::from("Unexpected EOF while parsing object (expected :)"),
			})
		};

		len += 1;

		if next != ':' {
			return Err(EonError {
				kind: EonErrorKind::ParseError,
				message: format!("Unexpected token while parsing object (expected :, got '{}'", next),
			});
		}

		len += advance_whitespace(&chars[len..]);

		let value = parse(&source[len..], &chars[len..])?;
		map.insert(String::from(key), value.0);
		len += value.1;

		len += advance_whitespace(&chars[len..]);

		let next = match chars.get(len) {
			Some(v) => *v,
			None => return Err(EonError {
				kind: EonErrorKind::ParseError,
				message: String::from("Unexpected EOF while parsing object (expected , or ')')"),
			})
		};

		len += 1;

		if next != ',' && next != ')' {
			return Err(EonError {
				kind: EonErrorKind::ParseError,
				message: format!("Unexpected token while parsing object (expected , or ')', got '{}')", next)
			});
		}

		if next == ',' {
			let whitespace = advance_whitespace(&chars[len..]);
			let next = match chars.get(len + whitespace) {
				Some(v) => *v,
				None => return Err(EonError {
					kind: EonErrorKind::ParseError,
					message: String::from("Unexpected EOF while parsing object (expected ')')"),
				})
			};

			if next == ')' {
				len += whitespace + 1;
				break;
			}
		}

		if next == ')' {
			break;
		}
	}

	Ok(Some((
		EonValue::Object(
			Box::new(EonObject(map))
		),
		len
	)))
}

pub fn parse_string(source: &str, chars: &[char]) -> Result<Option<(EonValue, usize)>> {
	let start = advance_whitespace(chars);
	let mut len = start;

	let delimiter = chars[start];
	if delimiter != '"' && delimiter != '\'' {
		return Ok(None);
	}

	len += 1;

	let mut previous = delimiter;
	for c in &chars[len..] {
		len += 1;

		if *c == delimiter && previous != '\\' {
			previous = *c;
			break;
		}

		previous = *c;
	}

	if previous != delimiter {
		return Err(EonError {
			kind: EonErrorKind::ParseError,
			message: String::from("Unexpected EOF while parsing string"),
		})
	}

	Ok(Some((
		EonValue::String(
			String::from(&source[start + 1..len - 1])
		),
		len,
	)))
}

fn advance_whitespace(chars: &[char]) -> usize {
	let mut len = 0;
	for c in chars {
		if c.is_whitespace() {
			len += 1;
		} else {
			break;
		}
	}

	len
}

fn is_name_special(c: char) -> bool {
	c == '!'
	|| ((c as u8) > 34 && (c as u8) < 39)
	|| c == '*'
	|| ((c as u8) > 44 && (c as u8) < 48)
	|| c == ':'
	|| c == '<'
	|| c == '>'
	|| c == '\\'
	|| c == '_'
	|| c == '~'
}

fn match_aplhanumeric<'a>(source: &'a str, chars: &[char]) -> &'a str {
	let mut len = 0;
	for c in chars {
		if c.is_alphanumeric() {
			len += 1;
		} else {
			break;
		}
	}

	&source[..len]
}

fn match_name<'a>(source: &'a str, chars: &[char]) -> &'a str {
	let mut len = 0;
	let mut first = true;
	for c in chars {
		if c.is_alphabetic() || is_name_special(*c) || (c.is_numeric() && !first) {
			len += 1;
		} else {
			break;
		}

		first = false;
	}

	&source[..len]
}

fn match_key<'a>(source: &'a str, chars: &[char]) -> &'a str {
	let mut len = 0;
	let mut first = true;
	for c in chars {
		if c.is_alphabetic() || *c == '_' || (c.is_numeric() && !first) {
			len += 1;
		} else {
			break;
		}

		first = false;
	}

	&source[..len]
}

fn match_number<'a>(source: &'a str, chars: &[char]) -> &'a str {
	let mut len = 0;
	let mut decimal = false;
	for c in chars {
		if c.is_numeric() {
			len += 1;
		} else if *c == '.' {
			if decimal {
				break;
			} else {
				len += 1;
				decimal = true;
			}
		} else {
			break;
		}
	}

	&source[..len]
}
