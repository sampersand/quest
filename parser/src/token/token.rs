use crate::Result;
use crate::stream::Stream;
use crate::token::{ParenType, Operator, Primitive, Tokenizable};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
	Primitive(Primitive),
	Operator(Operator),
	Left(ParenType),
	Right(ParenType),
	Endline,
	Comma
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Primitive(p) => Display::fmt(p, f),
			Self::Operator(o) => Display::fmt(o, f),
			Self::Left(t) => Display::fmt(&t.left(), f),
			Self::Right(t) => Display::fmt(&t.right(), f),
			Self::Endline => Display::fmt(&';', f),
			Self::Comma => Display::fmt(&',', f),
		}		
	}
}

/// parse whitespace that's not relevant.
fn parse_whitespace<S: Stream>(stream: &mut S) -> Result<()> {
	match stream.next().transpose()? {
		Some(chr) if chr.is_whitespace() =>
			while let Some(chr) = stream.next().transpose()? {
				if !chr.is_whitespace() {
					unseek_char!(stream; chr);
					return Ok(());
				}
			},
		Some(chr) => unseek_char!(stream; chr),
		None => {}
	}

	Ok(())
}

enum CommentResult {
	NoCommentFound,
	CommentRemoved,
	StopParsing
}

fn parse_comment<S: Stream>(stream: &mut S) -> Result<CommentResult> {
	fn parse_line<S: Stream>(stream: &mut S) -> Result<()> {
		while let Some(chr) = stream.next().transpose()? {
			if chr == '\n' {
				break;
			}
		}

		Ok(())
	}

	fn parse_block<S: Stream>(stream: &mut S) -> Result<()> {
		let begin_context = stream.context().clone();

		while let Some(chr) = stream.next().transpose()? {
			match chr {
				// end of line
				'*' if stream.next().transpose()? == Some('/') => return Ok(()),
				// allow for nested block comments
				'/' if stream.next().transpose()? == Some('*') => parse_block(stream)?,
				_ => { /* do nothing, we ignore other characters */ }
			}
		}

		Err(parse_error!(context=begin_context, UnterminatedBlockComment))
	}

	if stream.starts_with("##__EOF__##")? {
		Ok(CommentResult::StopParsing)
	} else if stream.starts_with("#")? {
		parse_line(stream).and(Ok(CommentResult::CommentRemoved))
	} else if stream.next_if_starts_with("/*")? {
		parse_block(stream).and(Ok(CommentResult::CommentRemoved))
	} else {
		Ok(CommentResult::NoCommentFound)
	}
}

impl Token {
	pub fn try_parse<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		parse_whitespace(stream)?;

		match parse_comment(stream)? {
			CommentResult::StopParsing => return Ok(None),
			CommentResult::CommentRemoved => return Self::try_parse(stream),
			CommentResult::NoCommentFound => {}
		}

		if let Some(prim) = Primitive::try_tokenize(stream)? {
			return Ok(Some(prim.into()))
		} else if let Some(op) = Operator::try_tokenize(stream)? {
			return Ok(Some(op.into()))
		}


		match stream.next().transpose()? {
			Some(';') => Ok(Some(Self::Endline)),
			Some(',') => Ok(Some(Self::Comma)),
			Some('(') => Ok(Some(Self::Left(ParenType::Round))),
			Some(')') => Ok(Some(Self::Right(ParenType::Round))),
			Some('[') => Ok(Some(Self::Left(ParenType::Square))),
			Some(']') => Ok(Some(Self::Right(ParenType::Square))),
			Some('{') => Ok(Some(Self::Left(ParenType::Curly))),
			Some('}') => Ok(Some(Self::Right(ParenType::Curly))),
			Some(chr) => Err(parse_error!(stream, UnknownTokenStart(chr))),
			None => Ok(None)
		}
	}
}
