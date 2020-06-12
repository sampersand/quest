use crate::{Stream, Result};
use crate::token::{Parsable, ParseResult};
use std::io::BufRead;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number(quest::types::Number);

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Number {
	fn try_parse_radix<S: BufRead>(_stream: &mut Stream<S>, _radix: u32) -> Result<ParseResult<Self>> {
		todo!("try_parse_radix")
	}

	fn try_parse_basic<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Self>> {
		let mut number = String::with_capacity(1);

		#[derive(PartialEq, Eq)]
		enum Position {
			Integer,
			Decimal,
			Mantissa
		}

		let mut pos = Position::Integer;

		while let Some(chr) = stream.next_char()? {
			match chr {
				'0'..='9' => number.push(chr),
				'.' if pos == Position::Integer => {
					// "12.x" should be '12' '.' 'x' is the start of an attribute
					if !matches!(stream.peek_char()?, Some('0'..='9')) {
						stream.unshift_char('.');
						break;
					}
					number.push('.');
					pos = Position::Decimal;
				},
				'e' | 'E' => if pos == Position::Mantissa {
					return Err(parse_error!(stream, BadNumber("unexpected `e` encountered".to_string())))
				} else {
					number.push('e');
					if matches!(stream.peek_char()?, Some('+') | Some('-')) {
						number.push(stream.next_char()?.unwrap());
					}
					pos = Position::Mantissa
				},
				_ => {
					stream.unshift_char(chr);
					break
				}
			}
		}

		quest::types::Number::try_from(number.as_str())
			.map_err(|err| parse_error!(stream, BadNumber(err.to_string())))
			.map(Number)
			.map(ParseResult::Some)
	}
}

impl Parsable for Number {
	type Item = Self;
	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Self>> {
		match stream.peek_char()? {
			Some('0') => {
				assert_eq!(stream.next_char()?, Some('0'));

				match stream.next_char()? {
					Some('x') | Some('X') => return Number::try_parse_radix(stream, 16),
					Some('d') | Some('D') => return Number::try_parse_radix(stream, 10),
					Some('o') | Some('O') => return Number::try_parse_radix(stream,  8),
					Some('b') | Some('B') => return Number::try_parse_radix(stream,  2),
					Some(chr) => stream.unshift_char(chr),
					None => {}
				}

				stream.unshift_char('0');
				Number::try_parse_basic(stream)
			},
			Some('1'..='9') => Number::try_parse_basic(stream),
			_ => Ok(ParseResult::None)
		}
	}
}