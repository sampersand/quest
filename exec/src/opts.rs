use quest::Object;
use quest_parser::{Stream, Expression, expression::Executable};
use crate::{Result, Repl, BufStream};
use std::convert::TryFrom;
use clap::Clap;
use std::path::PathBuf;

/// Run the Quest programming language
#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Sam Westerman <sam@sampersand.me>")]
pub(super) struct Opts {
	/// Define the file to run. If `-` is supplied, STDIN is read.
	#[clap(short="f", long, conflicts_with="eval")]
	file: Option<PathBuf>,

	/// Evaluate a passed command as quest code. Omit `file`.
	#[clap(short, long, conflicts_with="file")]
	eval: Option<String>,

	#[clap(last=true)]
	pub args: Vec<String>
	/*
	#[clap(short, long, env="QUEST_DEBUG")]
	debug: bool
	*/
}

impl Opts {
	pub fn run(self) -> Result<Object> {
		use std::io::{self, BufReader};

		let Opts { file, eval, .. } = self;
		match (file, eval) {
			(Some(_), Some(_)) => panic!("both options set?"),
			(Some(file), None) if file.to_str() == Some("-") =>
				Expression::parse_stream(
					BufStream::new(BufReader::new(io::stdin()), Some("-".into())).tokens())?,
			(Some(file), None) => Expression::parse_stream(BufStream::try_from(file.as_ref())?.tokens())?,
			(None, Some(eval)) => Expression::parse_stream(BufStream::from(eval).tokens())?,
			(None, None) => Expression::parse_stream(Repl::new().tokens())?
		}.execute().map_err(Into::into)

	}	
}



