//! Parsing a literal number.

use crate::{Result, Stream};
use crate::token::Tokenizable;
use crate::expression::Executable;
use std::convert::TryFrom;
use quest_core::Object;

/// A literal number is actually just a `quest_core::Number`.
pub use quest_core::types::Number;

/// The error that can occur whilst trying to parse a Number
pub use quest_core::types::number::FromStrError as ParseError;

impl Executable for Number {
	fn execute(&self) -> quest_core::Result<Object> {
		Ok(self.clone().into())
	}
}

/// Try to parse a number from the specified radix.
///
/// This function itself doesn't verify that values it reads are valid: we rely on `quest`'s
/// [`Number::from_str_radix`](#) to do that for us. As such, we just gobble up all the
/// alphanumeric values, ignoring underscores.
fn try_tokenize_radix<S: Stream>(stream: &mut S, radix: u32) -> Result<Number> {
	let mut number = String::with_capacity(1);

	while let Some(chr) = stream.next_non_underscore().transpose()? {
		match chr {
			chr @ '0'..='9'
				| chr @ 'a'..='z'
				| chr @ 'A'..='Z' => number.push(chr),
			chr => {
				// we've reached a non-number value, go back..
				unseek_char!(stream; chr);
				break
			}
		}
	}

	Number::from_str_radix(&number, radix)
		.map_err(|err| parse_error!(stream, BadNumber(err)))
}

/// This is a little more complex. To avoid using regex, the `Position` enum is used t
/// distinguish between different positions within the token.
///
/// Valid numbers should match the following regex:
/// ```regex
/// (?xi)
///   \d[\d_]*           # Position::Integer
///   (\.\d[\d_]*)?      # Position::Decimal
///   ([eE][+-]?[\d_]+\) # Position::Mantissa
/// ```
fn try_tokenize_basic<S: Stream>(stream: &mut S) -> Result<Number> {
	let mut number = String::with_capacity(1);

	#[derive(PartialEq)]
	enum Position { Integer, Decimal, Mantissa }

	let mut pos = Position::Integer;


	while let Some(chr) = stream.next_non_underscore().transpose()? {
		match chr {
			// no matter where we are, we always accept a decimal
			'0'..='9' => number.push(chr),
			// periods are only recognized during the `Integer` portion, **AND** if the following
			// character is a digit. If it's something else, eg '$', we should parse the period as
			// a distinct token. So, `12.3` would be '12.3', but `12.foo` would be '12' '.' 'foo'.
			'.' if pos == Position::Integer => {
				match stream.next().transpose()? {
					Some(digit @ '0'..='9') => {
						number.push('.');
						number.push(digit);
						pos = Position::Decimal;
					},
					Some(chr) => {
						// unseek both the current char and the `.`
						unseek_char!(stream; chr, '.');
						break;
					},
					// This means we have a dangling period. Let some other tokenizer deal with that,
					// and just happily parse our digit.
					None => {
						unseek_char!(stream; '.');
						break;
					}
				}
			},
			// reading a 'e' (or 'E') only is possible before the `Mantissa` section, and indicates
			// we're an exponential number.
			'e' | 'E' if pos != Position::Mantissa => {
				number.push('e');

				// Reead the optional `+` or `-` following an `e`
				match stream.next_non_underscore().transpose()? {
					Some(chr @ '+') | Some(chr @ '-') => number.push(chr),
					Some(chr) => unseek_char!(stream; chr),
					None => { /* do nothing */ }
				}

				pos = Position::Mantissa
			},
			chr => {
				// any other character indicates we're done looking
				unseek_char!(stream; chr);
				break
			}
		}
	}

	// Try to parse a number from what we've gotten.
	Number::try_from(number.as_str())
		.map_err(|err| parse_error!(stream, BadNumber(err)))
}

impl Tokenizable for Number {
	/// Try to parse a literal number from the given stream.
	///
	/// There are two forms of valid numbers: "radix-based" and "normal"
	///
	/// # Radix-Based
	///
	/// Radix-based numbers are, at first blush, what you'd expect to see: `0xef` for hex, `0o357`
	/// for octal and `0b11101111` for binary (all of which are case-insensitive; you could write
	/// `0XEF`, for example). However an additional `d` is added that specifies `decimal`---you could
	/// simply leave off the `0d`---which is provided for continuity: `0d239` is identical to `239`.
	///
	/// In the future, a way of specifying arbitrary bases via something like `0u<36>6n` may be
	/// possible, but for now that doesn't work.
	///
	/// # Normal
	/// 
	/// As Quest doesn't distinguish between integers and floats, the syntax for writing a "normal"
	/// number takes both forms into consideration: You can write normal integers (e.g. 12, 491,
	/// 001210---note leading zeroes don'y imply octal), normal floats (e.g. 12.34, 0.1, 9.140),
	/// and exponential notation (12e3, 19.1e-3, etc.)
	/// 
	/// Underscores are allowed in most places, where they are completely ignored. The only time an
	/// underscore is significant is directly after the `.` in floats (e.g. `12._3`), as that implies
	/// an element access, e.g. the tokens '12' '.' '_3'.
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {

		match stream.next().transpose()? {
			// If we find a zero, we could have `0x...` syntax
			Some('0') => {
				match stream.next().transpose()? {
					// FUTURE: Add in support for arbitrary bases, eg '0u<base>...'
					// Allow for literal hexadecimal numbers (which match /^0x[a-f\d_]+/i)
					Some('x') | Some('X') => try_tokenize_radix(stream, 16).map(Some),
					// Allow for literal decimal numbers (which match /^0d[\d_]+/i).
					// This is only here for parallel with the other branches, and probably wont be used.
					Some('d') | Some('D') => try_tokenize_radix(stream, 10).map(Some),
					// Allow for literal octal numbers (which match /^0o[0-7_]+/i)
					Some('o') | Some('O') => try_tokenize_radix(stream,  8).map(Some),
					// Allow for literal binary numbers (which match /^0b[01_]+/i)
					Some('b') | Some('B') => try_tokenize_radix(stream,  2).map(Some),
					// Any other trailing value indicates we're dealing with a number with a leading zero
					Some(chr) => {
						unseek_char!(stream; chr, '0');
						try_tokenize_basic(stream).map(Some)
					},
					// If we have no numbers remaining, we read a literal zero.
					None => Ok(Some(Number::ZERO)),
				}
			},

			// If we read a digit, then try parsing a basic number.
			Some(chr @ '1'..='9') => {
				unseek_char!(stream; chr);
				try_tokenize_basic(stream).map(Some)
			},

			// If we read anything else, it's not number.
			Some(chr) => {
				unseek_char!(stream; chr);
				Ok(None)
			},

			// If there's nothing left to read, we can't parse a number.
			None => Ok(None)
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::stream::BufStream;

	macro_rules! num {
		($n:expr) => { Number::from($n) };
	}

	macro_rules! buf {
		(*$n:expr) => { BufStream::from($n) };
		($n:expr) => { &mut buf!(*$n) };
	}

	#[test]
	fn is_number_executable() {
		fn requires_exec<X: Executable>(_: X){}
		#[allow(dead_code)]
		fn takes_num(n: Number) { requires_exec(n) }
	}

	mod radix {
		use super::*;
		use try_tokenize_radix as ttr;

		#[test]
		fn bad_base() {
			assert!(ttr(buf!("0"), 0).is_err());
			assert!(ttr(buf!("0"), 1).is_err());
			assert!(ttr(buf!("0"), 2).is_ok());
			assert!(ttr(buf!("0"), 36).is_ok());
			assert!(ttr(buf!("0"), 37).is_err());
			assert!(ttr(buf!("0"), 38).is_err());
		}

		#[test]
		fn normal() {
			// binary
			assert_eq!(num!(0), ttr(buf!("0"), 2).unwrap());
			assert_eq!(num!(1), ttr(buf!("1"), 2).unwrap());
			assert_eq!(num!(193), ttr(buf!("1100_0001"), 2).unwrap());
			assert_eq!(num!(17), ttr(buf!("10__0_01__"), 2).unwrap());
			assert!(ttr(buf!("2"), 2).is_err());

			// octal
			assert_eq!(num!(0), ttr(buf!("0"), 8).unwrap());
			assert_eq!(num!(7), ttr(buf!("7"), 8).unwrap());
			assert_eq!(num!(193), ttr(buf!("301"), 8).unwrap());
			assert_eq!(num!(17), ttr(buf!("2__1__"), 8).unwrap());
			assert!(ttr(buf!("8"), 8).is_err());

			// decimal
			assert_eq!(num!(0), ttr(buf!("0"), 10).unwrap());
			assert_eq!(num!(9), ttr(buf!("9"), 10).unwrap());
			assert_eq!(num!(193), ttr(buf!("193"), 10).unwrap());
			assert_eq!(num!(17), ttr(buf!("1__7__"), 10).unwrap());
			assert!(ttr(buf!("a"), 10).is_err());

			// hexadecimal
			assert_eq!(num!(0), ttr(buf!("0"), 16).unwrap());
			assert_eq!(num!(15), ttr(buf!("f"), 16).unwrap());
			assert_eq!(num!(15), ttr(buf!("F"), 16).unwrap());
			assert_eq!(num!(193), ttr(buf!("c1"), 16).unwrap());
			assert_eq!(num!(17), ttr(buf!("1__1__"), 16).unwrap());
			assert!(ttr(buf!("g"), 16).is_err());
		}

		#[test]
		fn empty() {
			assert!(ttr(buf!(""), 10).is_err());
			assert!(ttr(buf!(" "), 16).is_err());
			assert!(ttr(buf!("\n12"), 16).is_err());
		}

		#[test]
		fn afterwards() {
			let buf = buf!("45.3");
			assert_eq!(num!(45), ttr(buf, 10).unwrap());
			assert_eq!(buf.next().unwrap().unwrap(), '.');

			let buf = buf!("45\n12");
			assert_eq!(num!(45), ttr(buf, 10).unwrap());
			assert_eq!(buf.next().unwrap().unwrap(), '\n');
		}
	}

	mod basic {
		use super::*;
		use try_tokenize_basic as ttb;

		#[test]
		fn integers() {
			assert_eq!(num!(0), ttb(buf!("0")).unwrap());
			assert_eq!(num!(0), ttb(buf!("00_00")).unwrap());
			assert_eq!(num!(1), ttb(buf!("00_001")).unwrap());
			assert_eq!(num!(1_234_567), ttb(buf!("1__2_34_56__7")).unwrap());
		}

		#[test]
		fn decimal() {
			assert_eq!(num!(0), ttb(buf!("0.0")).unwrap());
			assert_eq!(num!(12), ttb(buf!("12.00")).unwrap());
			assert_eq!(num!(12.01), ttb(buf!("12.0100")).unwrap());
			assert_eq!(num!(0.1234), ttb(buf!("0.1234_")).unwrap());
			assert_eq!(num!(12.34), ttb(buf!("12_.3___4")).unwrap());
			assert_eq!(num!(12), ttb(buf!("12_.00")).unwrap());
		}

		#[test]
		fn exponent() {
			assert_eq!(num!(12e3), ttb(buf!("12e3")).unwrap());
			assert_eq!(num!(12.34e2), ttb(buf!("1_2_.3_4_e2")).unwrap());
			assert_eq!(num!(12.34e2), ttb(buf!("1_2_.3_4_e+2")).unwrap());
			assert_eq!(num!(12.34e-2), ttb(buf!("1_2_.3_4_e-2")).unwrap());	
		}

		// make sure a '.' is being parsed as a decimal separator and as an attr accessor correctly.
		#[test]
		fn decimal_vs_accessor() {
			// make sure periods just work in general
			let buf = buf!("12.3*4");
			assert_eq!(num!(12.3), ttb(buf).unwrap());
			assert_eq!('*', buf.next().unwrap().unwrap());

			// make sure they work on flat integers
			let buf = buf!("12.foo");
			assert_eq!(num!(12), ttb(buf).unwrap());
			assert_eq!('.', buf.next().unwrap().unwrap());
			assert_eq!('f', buf.next().unwrap().unwrap());

			// make sure they work on decimal numebrs
			let buf = buf!("12.34.foo");
			assert_eq!(num!(12.34), ttb(buf).unwrap());
			assert_eq!('.', buf.next().unwrap().unwrap());
			assert_eq!('f', buf.next().unwrap().unwrap());

			// ... including when those trailing characters are digits
			let buf = buf!("12.34.foo");
			assert_eq!(num!(12.34), ttb(buf).unwrap());
			assert_eq!('.', buf.next().unwrap().unwrap());
			assert_eq!('f', buf.next().unwrap().unwrap());

			// make sure they work for exponents
			let buf = buf!("12e3.foo");
			assert_eq!(num!(12e3), ttb(buf).unwrap());
			assert_eq!('.', buf.next().unwrap().unwrap());
			assert_eq!('f', buf.next().unwrap().unwrap());

			let buf = buf!("12.34e5.12");
			assert_eq!(num!(12.34e5), ttb(buf).unwrap());
			assert_eq!('.', buf.next().unwrap().unwrap());
			assert_eq!('1', buf.next().unwrap().unwrap());
		}

		// make sure a trailing '.' isn't gobbled up on accident
		#[test]
		fn no_trailing_period() {
			let buf = buf!("12.");
			assert_eq!(num!(12), ttb(buf).unwrap());
			assert_eq!('.', buf.next().unwrap().unwrap());
			assert_eq!(None, buf.next().transpose().unwrap());
		}
	}

	mod tokenizable {
		use super::*;

		fn tkn<S: Stream>(stream: &mut S) -> Option<Number> {
			Number::try_tokenize(stream).unwrap()
		}

		#[test]
		fn before() {
			assert_eq!(tkn(buf!("")), None);
			assert_eq!(tkn(buf!(" ")), None);
			assert_eq!(tkn(buf!(" 1")), None);
			assert_eq!(tkn(buf!("_12")), None);
			assert_eq!(tkn(buf!(".34")), None);
			assert_eq!(tkn(buf!("\n34")), None);
		}

		#[test]
		fn valid() {
			assert_eq!(tkn(buf!("0")), Some(num!(0)));
			assert_eq!(tkn(buf!("02")), Some(num!(2)));
			assert_eq!(tkn(buf!("02")), Some(num!(2)));
			assert_eq!(tkn(buf!("4.1e-4")), Some(num!(4.1e-4)));
		}

		#[test]
		fn after() {
			let buf = buf!("4.1.2");
			assert_eq!(tkn(buf), Some(num!(4.1)));
			assert_eq!(buf.next().unwrap().unwrap(), '.');
			assert_eq!(buf.next().unwrap().unwrap(), '2');

			let buf = buf!("0 =");
			assert_eq!(tkn(buf), Some(num!(0)));
			assert_eq!(buf.next().unwrap().unwrap(), ' ');
			assert_eq!(buf.next().unwrap().unwrap(), '=');

			let buf = buf!("4.1e3e3");
			assert_eq!(tkn(buf), Some(num!(4.1e3)));
			assert_eq!(buf.next().unwrap().unwrap(), 'e');
			assert_eq!(buf.next().unwrap().unwrap(), '3');

			let buf = buf!("4.1e3.5");
			assert_eq!(tkn(buf), Some(num!(4.1e3)));
			assert_eq!(buf.next().unwrap().unwrap(), '.');
			assert_eq!(buf.next().unwrap().unwrap(), '5');

			let buf = buf!("4._2");
			assert_eq!(tkn(buf), Some(num!(4)));
			assert_eq!(buf.next().unwrap().unwrap(), '.');
			assert_eq!(buf.next().unwrap().unwrap(), '_');

			let buf = buf!("4.e");
			assert_eq!(tkn(buf), Some(num!(4)));
			assert_eq!(buf.next().unwrap().unwrap(), '.');
			assert_eq!(buf.next().unwrap().unwrap(), 'e');
		}
	}
}
