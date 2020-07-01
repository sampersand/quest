pub use quest_parser::stream::BufStream;
use std::io::BufRead;

impl<B: BufRead> crate::run::Runner for BufStream<B> {
	fn run(self) -> crate::Result<quest_core::Object> {
		use quest_parser::{Stream, Expression, expression::Executable};

		Expression::parse_stream(self.tokens())?
			.execute()
			.map_err(Into::into)
	}
}
