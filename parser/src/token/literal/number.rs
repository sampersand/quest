use crate::{Result, Stream, Error};
use crate::token::{Tokenizable, TokenizeResult};
use crate::expression::Executable;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

pub type Number = quest::types::Number;

impl Executable for Number {
	fn execute(&self) -> quest::Result<quest::Object> {
		Ok(self.clone().into())
	}
}

fn try_tokenize_radix<S: Stream>(stream: &mut S, radix: u32) -> Result<TokenizeResult<Number>> {
	let mut number = String::with_capacity(1);

	while let Some(chr) = stream.next().transpose()? {
		match chr {
			'_' => {},
			'0'..='9' | 'a'..='z' | 'A'..='Z' => number.push(chr),
			_ => { try_seek!(stream, -1); break }
		}
	}

	Number::from_str_radix(&number, radix)
		.map_err(|err| parse_error!(stream, BadNumber(err.to_string())))
		.map(TokenizeResult::Some)
}

fn try_tokenize_basic<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Number>> {
	let mut number = String::with_capacity(1);

	#[derive(PartialEq)]
	enum Position {
		Integer,
		Decimal,
		Mantissa
	}

	let mut pos = Position::Integer;

	while let Some(chr) = stream.next().transpose()? {
		match chr {
			'0'..='9' => number.push(chr),
			'.' if pos == Position::Integer => {
				// "12.\D" should be interpreted as '12' '.' '\D', as '\D' is the start of an attr
				// (\D means "not 0-9")
				if matches!(stream.peek().transpose()?, Some('0'..='9')) {
					number.push('.');
					pos = Position::Decimal;
				} else {
					try_seek!(stream, -1);
					break
				}
			},

			'e' | 'E' => if pos == Position::Mantissa {
				return Err(parse_error!(stream, BadNumber("unexpected `e` encountered".to_string())))
			} else {
				number.push('e');
				// accept the sign of the exponent, if supplied
				if matches!(stream.peek().transpose()?, Some('+') | Some('-')) {
					number.push(stream.next().transpose()?.unwrap());
				}
				pos = Position::Mantissa
			},
			_ => {
				try_seek!(stream, -1);
				break
			}
		}
	}

	Number::try_from(number.as_str())
		.map_err(|err| parse_error!(stream, BadNumber(err.to_string())))
		.map(TokenizeResult::Some)
}

impl Tokenizable for Number {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match stream.peek().transpose()? {
			Some('0') => {
				assert_eq!(stream.next().transpose()?, Some('0'));

				match stream.next().transpose()? {
					Some('x') | Some('X') => try_tokenize_radix(stream, 16),
					Some('d') | Some('D') => try_tokenize_radix(stream, 10),
					Some('o') | Some('O') => try_tokenize_radix(stream,  8),
					Some('b') | Some('B') => try_tokenize_radix(stream,  2),
					Some(_) => {
						try_seek!(stream, -2);
						try_tokenize_basic(stream)
					},
					None => Ok(TokenizeResult::Some(Number::ZERO)),
				}
			},
			Some('1'..='9') => try_tokenize_basic(stream),
			_ => Ok(TokenizeResult::None)
		}
	}
}


