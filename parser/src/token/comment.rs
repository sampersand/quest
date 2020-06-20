pub use super::whitespace::Never;
use crate::token::{Tokenizable, TokenizeResult};
use crate::{Stream, Result};

// a dummy struct just so we can have a type to impl `Tokenizable`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment;

fn line_comment<S: Stream>(stream: &mut S) -> Result<()> {
	while let Some(chr) = stream.next().transpose()? {
		if chr == '\n' {
			break;
		}
	}
	Ok(())
}

fn block_comment<S: Stream>(stream: &mut S) -> Result<()> {
	let begin_context = stream.context().clone();

	while let Some(chr) = stream.next().transpose()? {
		match chr {
			// end of line
			'*' if stream.next().transpose()? == Some('/') => return Ok(()),
			// allow for nested block comments
			'/' if stream.next().transpose()? == Some('*') => block_comment(stream)?,
			_ => { /* do nothing, we ignore other characters */ }
		}
	}

	Err(parse_error!(context=begin_context, UnterminatedBlockComment))
}

impl Tokenizable for Comment {
	type Item = Never;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Never>> {
		if stream.starts_with("##__EOF__##")? {
			Ok(TokenizeResult::StopParsing)
		} else if stream.starts_with("#")? {
			line_comment(stream).and(Ok(TokenizeResult::RestartParsing))
		} else if stream.starts_with("/*")? {
			try_seek!(stream, 2);
			block_comment(stream).and(Ok(TokenizeResult::RestartParsing))
		} else {
			Ok(TokenizeResult::None)
		}
	}
}






