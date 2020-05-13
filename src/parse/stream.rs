use std::iter::Peekable;
use crate::parse::{Result, Error, token::{self, Literal}, Token};
use crate::obj::types;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Bytes, BufReader, Seek, SeekFrom, Cursor};

#[derive(Debug)]
pub struct Stream<'a, S: Seek + Read> {
	data: S,
	file: Option<&'a Path>,
	line: usize,
	col: usize,
	row: usize,
}

impl<'a, S: Seek + Read> Stream<'a, S> {
	pub fn new(data: S, file: Option<&'a Path>) -> Self {
		Stream { data, file, line: 0, col: 0, row: 0 }
	}

	// TODO: this doesn't actually get the next char; it doesn't allow unicode characters.
	fn next_char(&mut self) -> io::Result<Option<char>> {
		let mut c: u8 = 0;
		match self.data.read(std::slice::from_mut(&mut c)) {
			Ok(0) => Ok(None),
			Ok(..) => Ok(Some(char::from(c))),
			Err(err) => Err(err)
		}
	}

	fn unseek(&mut self, chr: char) -> io::Result<()> {
		self.data.seek(SeekFrom::Current(-(chr.len_utf8() as i64))).and(Ok(()))
	}
}

impl<'a> Stream<'static, Cursor<&'a str>> {
	pub fn from_str(data: &'a str) -> Self {
		Stream::new(Cursor::new(data), None)
	}
}

impl<'a> Stream<'a, BufReader<File>> {
	pub fn from_file(file: &'a Path) -> io::Result<Self> {
		Ok(Stream::new(BufReader::new(File::open(file)?), Some(file.as_ref())))
	}
}

macro_rules! parse_err {
	($($msg:expr),* $(,)?) => {
		return Err(Error::Message(format!($($msg),*)))
	};
}

impl<'a, S: Seek + Read> Stream<'a, S> {
	fn next_variable(&mut self, first_chr: char) -> Result<Token> {
		let mut var = first_chr.to_string();

		while let Some(chr) = self.next_char()? {
			if chr.is_alphanumeric() || chr == '_' {
				var.push(chr);
			} else {
				self.unseek(chr);
				break;
			}
		}

		Ok(Token::Literal(Literal::Variable(types::Text::new(var))))
	}

	fn next_number_radix(&mut self, radix: u32) -> Result<Token> {
		todo!();
	}

	fn next_number(&mut self, first_chr: char) -> Result<Token> {
		let mut num = first_chr.to_string();

		if first_chr == '0' {
			match self.next_char()? {
				Some('b') | Some('B') => return self.next_number_radix(2),
				Some('o') | Some('O') => return self.next_number_radix(8),
				Some('d') | Some('D') => return self.next_number_radix(10),
				Some('x') | Some('X') => return self.next_number_radix(16),
				None => return Ok(Token::Literal(Literal::Number(types::number::ZERO))),
				Some(chr) => self.unseek(chr)?
			}
		}

		#[derive(PartialEq)]
		enum Stage { Whole, Decimal, Exponent };

		let mut stage = Stage::Whole;

		while let Some(chr) = self.next_char()? {
			match chr {
				'0'..='9' => num.push(chr),
				'_' => { /* do nothing, ignore underscores */ }
				'.' if stage == Stage::Whole => {
					stage = Stage::Decimal;
					match self.next_char()? {
						None => parse_err!("trailing period: {:?}", num),
						Some(digit @ '0'..='9') => {
							num.push('.');
							num.push(digit);
						},
						Some(alnum) if alnum.is_alphanumeric() => {
							self.unseek(alnum);
							self.unseek('.');
							break;
						},
						Some(other) => parse_err!("trailing period: {:?}", num)
					}
				},
				'e' | 'E' if stage != Stage::Exponent => {
					stage = Stage::Exponent;
					num.push(chr);
					match self.next_char()? {
						Some(chr @ '+')
							| Some(chr @ '-')
							| Some(chr @ '0'..='9') => num.push(chr),
						Some(_) | None => parse_err!("bad exponent trailing val"),
					}
				},
				_ if chr.is_alphabetic() => parse_err!("bad digit: {}", chr),
				_ => { self.unseek(chr); break }
			}
		}

		// TODO: make this actually call the `from_str` method
		Ok(Token::Literal(Literal::Number(types::Number::from_str(&num)
				.map_err(|err| Error::Message(format!("bad number {:?}: {}", num, err)))?)))
	}

	fn next_text(&mut self, quote: char) -> Result<Token> {
		let mut txt = String::new();
		while let Some(chr) = self.next_char()? {
			txt.push(match chr {
				'\\' => match self.next_char()?.ok_or(Error::UnterminatedQuote)? {
					chr @ '\\' | chr @ '\'' | chr @ '\"' => '\\',
					'n' => '\n',
					't' => '\t',
					'r' => '\r',
					'0' => '\0',
					'u' | 'U' | 'x' | '0'..='9' => todo!("additional string parsing"),
					_ => parse_err!("unknown escape: {}", chr)
				},
				_ if chr == quote => return Ok(Token::Literal(Literal::Text(types::Text::from(txt)))),
				_ => chr
			});
		}
		parse_err!("unterminated quote")
	}

	fn next_operator(&mut self, first_chr: char) -> Result<Token> {
		macro_rules! following {
			($op:ident) => { super::token::Operator::$op };
			($op:ident; ) => { following!($op) };
			(; $expr:expr) => { $expr };
			($op:ident $($chr:literal $($chr_op:ident)? $(=> $expr:expr)?),*) => {
				if let Some(chr) = self.next_char()? {
					match chr {
						$($chr => { following!($($chr_op)?; $($expr)?) }),*
						other => { self.unseek(chr)?; super::token::Operator::$op }
					}
				} else {
					super::token::Operator::$op
				}
			};
		}
		Ok(Token::Operator(match first_chr {
			'+' => following!(Add '=' AddAsn, '@' Pos),
			'-' => following!(Sub '=' SubAsn, '@' Neg),
			'*' => following!(Mul '=' MulAsn, '*' => following!(Pow '=' PowAsn)),
			'%' => following!(Mod '=' ModAsn),
			'/' => following!(Div '=' DivAsn),

			'!' => following!(Not '=' Neq),
			'=' => following!(Assign '=' Eql),
			'<' => following!(Lth '=' => following!(Leq '>' Cmp), '<' => following!(Lsh '=' LshAsn)),
			'>' => following!(Gth '=' Geq, '>' => following!(Rsh '=' RshAsn)),

			'~' => following!(BNot),
			'&' => following!(BAnd '=' BAndAsn, '&' And),
			'|' => following!(BOr '=' BOrAsn, '|' Or),
			'^' => following!(Xor '=' XorAsn),

			'.' => following!(Dot '=' DotAsn, '~' DotDel),
			',' => following!(Comma),
			';' => following!(Endline),

			':' | '$' | '?' | '@' | '\\' | '`' => return Err(Error::UnknownTokenStart(first_chr)),
			_ => unreachable!()
		}))
	}
}



impl<'a, S: Seek + Read> Iterator for Stream<'a, S> {
	type Item = Result<Token>;
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
			_ if chr.is_alphabetic() || chr == '_' => self.next_variable(chr),
			'\'' | '"' => self.next_text(chr),

			'(' => Ok(Token::Left(token::ParenType::Paren)),
			'[' => Ok(Token::Left(token::ParenType::Bracket)),
			'{' => Ok(Token::Left(token::ParenType::Curly)),
			')' => Ok(Token::Right(token::ParenType::Paren)),
			']' => Ok(Token::Right(token::ParenType::Bracket)),
			'}' => Ok(Token::Right(token::ParenType::Curly)),

			'\\' => todo!("line continuation"),
			// punctuation characters not covered before:
			// 	! $ % & * + , - . / : ; < = > ? @ \ ^ ` , | ~
			// not all of them are actually used as variables
			_ if chr.is_ascii_punctuation() => self.next_operator(chr),

			_ => Err(Error::UnknownTokenStart(chr))
		})
	}
}








