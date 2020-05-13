use std::iter::Peekable;
use crate::parse::{self, token, Token};
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Bytes, BufReader, Cursor};

#[derive(Debug)]
pub struct Stream<'a, T: Read> {
	data: Peekable<Bytes<T>>,
	file: Option<&'a Path>,
	line: usize,
	col: usize,
	row: usize,
}

impl<'a, T: Read> Stream<'a, T> {
	pub fn new(data: T, file: Option<&'a Path>) -> Self {
		Stream { data: data.bytes().peekable(), file, line: 0, col: 0, row: 0 }
	}

	// TODO: this doesn't actually get the next char; it doesn't allow unicode characters.
	fn next_char(&mut self) -> io::Result<Option<char>> {
		self.data.next().transpose().map(|x| x.map(char::from))
	}

	// TODO: this doesn't actually get the next char; it doesn't allow unicode characters.
	fn peek_next_char(&mut self) -> io::Result<Option<char>> {
		match self.data.peek() {
			None => Ok(None),
			Some(Ok(ref val)) => Ok(Some(char::from(*val))),
			Some(Err(ref err)) => Err(io::Error::new(err.kind(), "<err>"))
		}
	}
}

impl<'a> Stream<'static, BufReader<Cursor<&'a str>>> {
	pub fn from_str(data: &'a str) -> Self {
		Stream::new(BufReader::new(Cursor::new(data)), None)
	}
}

impl<'a> Stream<'a, BufReader<File>> {
	pub fn from_file(file: &'a Path) -> io::Result<Self> {
		Ok(Stream::new(BufReader::new(File::open(file)?), Some(file.as_ref())))
	}
}

impl<'a, T: Read> Stream<'a, T> {
	fn next_variable(&mut self, first_chr: char) -> parse::Result<Token> {
		let mut var = first_chr.to_string();

		while let Some(chr) = self.peek_next_char()? {
			if chr.is_alphanumeric() || chr == '_' {
				var.push(self.next_char()?.expect("we just got this?"));
			} else {
				break
			}
		}

		Ok(Token::Variable(var))
	}

	fn next_number(&mut self, first_chr: char) -> parse::Result<Token> {
		let mut num = first_chr.to_string();
		enum Location {
			Beginning,
			Radix(u32),
			Integer,
			Decimal,
			Exponent,
		}

		use Location::*;

		let mut location = Beginning;
		macro_rules! add_next_char {
			() => { num.push(self.next_char()?.expect("we just got this?")) };
		}
		while let Some(chr) = self.peek_next_char()? {
			match (location, chr.to_lowercase().next().expect("at least 1 should always return?")) {
				(Beginning, 'b') => { location = Radix(2); continue },
				(Beginning, 'o') => { location = Radix(8); continue },
				(Beginning, 'd') => { location = Radix(10); continue },
				(Beginning, 'x') => { location = Radix(16); continue },
				(Radix(2), '0'..='1')
					| (Radix(8), '0'..='7') 
					| (Radix(10), '0'..='9')
					| (Radix(16), '0'..='9')
					| (Radix(16), 'a'..'f') => add_next_char!(),

				(Radix(2), '0'..='1') => add_next_char!(),
				_ => break
			};
			num.push(self.next_char()?.expect("we just got this?"))
		}

		// let radix = if first_chr == '0' {
		// 	match self.peek_next_char()? {
		// 		Some('b') | Some('B') => Some(2),
		// 		Some('o') | Some('O') => Some(8),
		// 		Some('d') | Some('D') => Some(10),
		// 		Some('x') | Some('X') => Some(16),
		// 		_ => None
		// 	} 
		// } else {
		// 	None
		// };

		// enum Location { PreDecimal, PostDecimal, PostScientific, PostScientificDecimal };
		// let mut location = Location::PreDecimal;

		// while let Some(chr) = self.peek_next_char()? {
		// 	match chr {
		// 		'_' | '0'..='9' => {},
		// 		'.' if Location::PreDecimal => location = Location::PostDecimal,
		// 		'.' if Location::PostScientific => location = Location::PostScientificDecimal,
		// 		'e' | 'E' if Location::PreDecimal || Location::PreScientific
		// 		_ => break
		// 	};
		// 	var.push(self.next_char()?.expect("we just got this?"))
		// }

		Ok(Token::Number(num, None))
	}

	fn next_text(&mut self, quote: char) -> parse::Result<Token> {
		let mut txt = String::new();
		while let Some(chr) = self.next_char()? {
			txt.push(match chr {
				'\\' => match self.next_char()?.ok_or_else(|| parse::Error::UnterminatedQuote)? {
					chr @ '\\' | chr @ '\'' | chr @ '\"' => '\\',
					'n' => '\n',
					't' => '\t',
					'r' => '\r',
					'0' => '\0',
					'u' | 'U' | 'x' | '0'..='9' => todo!("additional string parsing"),
					_ => return Err(parse::Error::UnknownEscape(chr))
				},
				_ if chr == quote => return Ok(Token::Text(txt)),
				_ => chr
			});
		}
		Err(parse::Error::UnterminatedQuote)
	}
}



impl<'a, T: Read> Iterator for Stream<'a, T> {
	type Item = parse::Result<Token>;
	fn next(&mut self) -> Option<Self::Item> {
		// TODO: allow non-ascii characters?
		let chr = match self.next_char().transpose()? {
			Ok(chr) => chr,
			Err(err) => return Some(Err(err.into()))
		};

		Some(match chr {
			_ if chr.is_whitespace() => self.next()?,
			'#' => loop {
				match self.next_char() {
					Err(err) => return Some(Err(err.into())),
					Ok(Some('\n')) => return self.next(),
					Ok(Some(_)) => continue,
					Ok(None) => return None
				}
			},

			'0'..='9' => self.next_number(chr),
			_ if chr.is_alphanumeric() || chr == '_' => self.next_variable(chr),
			'\'' | '"' => self.next_text(chr),

			// there are other operators in the future, todo: them.
			'+' | '-' | '*' | '/' | '.' | '=' | '<' | '>'
				| '.' | ';' | ',' => Ok(Token::Operator(chr.to_string())),

			'(' => Ok(Token::Left(token::ParenType::Paren)),
			'[' => Ok(Token::Left(token::ParenType::Bracket)),
			'{' => Ok(Token::Left(token::ParenType::Curly)),
			')' => Ok(Token::Right(token::ParenType::Paren)),
			']' => Ok(Token::Right(token::ParenType::Bracket)),
			'}' => Ok(Token::Right(token::ParenType::Curly)),
			_ => Err(parse::Error::UnknownTokenStart(chr))
		})

		// 		None => token = Some(match chr {
		// 			'0'..='9' => Token::Number(chr.to_string(), None),
		// 			'a'..='z' | 'A'..='Z' | '_' => Token::Text(chr.to_string(), Quote::Variable),
		// 			'\'' => Token::Text(String::new(), Quote::Single),
		// 			'"'  => Token::Text(String::new(), Quote::Double),

		// 			_ => unimplemented!()
		// 		}),
		// 		Some(Token::Text(ref mut tkn, quote)) => match (quote, chr) {
		// 			(Quote::Variable, 'a'..='z')
		// 				| (Quote::Variable, 'A'..='Z')
		// 				| (Quote::Variable, '_')
		// 				| (Quote::Variable, '0'..='9') => tkn.push(chr),
		// 			(Quote::Variable, _) => break,
		// 			(Quote::Single, _) => unimplemented!(),
		// 			(Quote::Double, _) => unimplemented!(),
		// 			},
		// 		Some(Token::Number(ref mut tkn)) => unimplemented!(),
		// 		Some(Token::Operator(ref mut tkn)) => unimplemented!(),
		// 		Some(Token::Left(_)) | Some(Token::Right(_)) => unreachable!("left/right parens encountered!")
		// 	}
		// };

		// token.map(Result::Ok)
	}
}








