use crate::{Result, Stream};
use crate::token::{Tokenizable, TokenizeResult};
use crate::expression::Executable;
use std::convert::TryFrom;
use quest::{Object, types::Number};

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct NumberTokenizer;

impl Executable for Number {
	fn execute(&self) -> quest::Result<Object> {
		Ok(self.clone().into())
	}
}

impl NumberTokenizer {
	fn try_tokenize_radix<S: Stream>(stream: &mut S, radix: u32) -> Result<TokenizeResult<Number>> {
		let mut number = String::with_capacity(1);

		while let Some(chr) = stream.next().transpose()? {
			match chr {
				'_' => {},
				'0'..='9' | 'a'..='z' | 'A'..='Z' => number.push(chr),
				_ => { try_seek!(stream, -1); break }
			}
		}

		quest::types::Number::from_str_radix(&number, radix)
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
					match stream.next().transpose()? {
						Some(digit @ '0'..='9') => {
							number.push('.');
							number.push(digit);
							pos = Position::Decimal;
						},
						_ => {
							try_seek!(stream, -2);
							break
						}
					}
				},

				'e' | 'E' if pos == Position::Mantissa =>
					return Err(parse_error!(stream, BadNumber("unexpected `e` encountered".to_string()))),
				'e' | 'E' => {
					number.push('e');
					match stream.next().transpose()? {
						Some(chr @ '+') | Some(chr @ '-') => number.push(chr),
						Some(_) => try_seek!(stream, -1),
						_ => {}
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
}

impl Tokenizable for NumberTokenizer {
	type Item = Number;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Number>> {
		match stream.next().transpose()? {
			Some('0') => {
				match stream.next().transpose()? {
					Some('x') | Some('X') => NumberTokenizer::try_tokenize_radix(stream, 16),
					Some('d') | Some('D') => NumberTokenizer::try_tokenize_radix(stream, 10),
					Some('o') | Some('O') => NumberTokenizer::try_tokenize_radix(stream,  8),
					Some('b') | Some('B') => NumberTokenizer::try_tokenize_radix(stream,  2),
					Some(_) => {
						try_seek!(stream, -2);
						NumberTokenizer::try_tokenize_basic(stream)
					},
					None => Ok(TokenizeResult::Some(Number::ZERO)),
				}
			},
			Some('1'..='9') => {
				try_seek!(stream, -1);
				NumberTokenizer::try_tokenize_basic(stream)
			},
			Some(_) => {
				try_seek!(stream, -1);
				Ok(TokenizeResult::None)
			},
			None => Ok(TokenizeResult::None)
		}
	}
}
