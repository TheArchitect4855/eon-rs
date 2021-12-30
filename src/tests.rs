use crate::{EonValue, parse};

#[test]
fn test_parse_bool() {
	let a = super::parse("  true", )
		.unwrap();
	
	assert_eq!(a, EonValue::Bool(true));

	let b = super::parse("false  ")
		.unwrap();

	assert_eq!(b, EonValue::Bool(false));
}

#[test]
fn test_parse_number() {
	let int = super::parse("69")
		.unwrap();
	
	assert_eq!(int, EonValue::Number(69.0));

	let real = super::parse("3.14159")
		.unwrap();
	
	assert_eq!(real, EonValue::Number(3.14159));
}

#[test]
fn test_parse_name() {
	let name = super::parse("this_is-a:name")
		.unwrap();

	assert_eq!(name, EonValue::Name(
		String::from("this_is-a:name")
	));
}

#[test]
fn test_parse_string() {
	let string = super::parse("'This is a string!'")
		.unwrap();

	assert_eq!(string, EonValue::String(
		String::from("This is a string!")
	));

	let eof = super::parse("'This string is missing a closing quote");
	assert!(eof.is_err());
}

#[test]
fn test_parse_array() {
	let array = super::parse("[1, two, 'three', ]")
		.unwrap();

	assert_eq!(array, EonValue::Array(Box::from([
		EonValue::Number(1.0),
		EonValue::Name(
			String::from("two")
		),
		EonValue::String(
			String::from("three")
		),
	])));

	super::parse("
	[
		[1, 2, 3],
		[4, 5, 6]
	]
	").unwrap();
}

#[test]
fn test_parse_object() {
	super::parse("
	(
		name: 'Kurtis',
		age: 19,
		birthday: { 12/05/2002 },
		height_type: metric,
		height: 185.0
	)
	").unwrap();
}

#[test]
fn test_the_gauntlet() {
	let object = "
	(
		typedef: (
			name: string_array,
			age: int,
			height: (
				feet: int,
				inches: int
			),
			weight: float,
			constructor: function,
		),
		person: (
			name: ['Kurtis', 'Knodel'],
			age: 19,
			height: (
				feet: 6,
				inches: 1
			),
			weight: 200.6,
			constructor: {
				fn new() -> Self \\{
					// Do stuff
				\\}
			}
		)
	)
	";

	let object = parse(object)
		.unwrap();
	
	let string = object.to_string();
	parse(&string)
		.unwrap();
}