use crate::{Result, Stream};
use crate::token::{Tokenizable, TokenizeResult};
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number(quest::types::Number);

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Number {
	#[allow(unused_variables)]
	fn try_tokenize_radix<S: Stream>(stream: &mut S, radix: u32) -> Result<TokenizeResult<Self>> {
		todo!("try_tokenize_radix")
	}

	fn try_tokenize_basic<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
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

		quest::types::Number::try_from(number.as_str())
			.map_err(|err| parse_error!(stream, BadNumber(err.to_string())))
			.map(Number)
			.map(TokenizeResult::Some)
	}
}

impl Tokenizable for Number {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match stream.peek().transpose()? {
			Some('0') => {
				assert_eq!(stream.next().transpose()?, Some('0'));

				match stream.next().transpose()? {
					Some('x') | Some('X') => Number::try_tokenize_radix(stream, 16),
					Some('d') | Some('D') => Number::try_tokenize_radix(stream, 10),
					Some('o') | Some('O') => Number::try_tokenize_radix(stream,  8),
					Some('b') | Some('B') => Number::try_tokenize_radix(stream,  2),
					Some(_) => {
						try_seek!(stream, -2);
						Number::try_tokenize_basic(stream)
					},
					None => Ok(TokenizeResult::Some(Number(0.into()))),
				}
			},
			Some('1'..='9') => Number::try_tokenize_basic(stream),
			_ => Ok(TokenizeResult::None)
		}
	}
}


