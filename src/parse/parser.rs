use super::Stream;
use std::io::{Seek, Read};

pub struct Parser<'a, S: Seek + Read> {
	stream: Stream<'a, S>
}

impl<'a, S: Seek + Read> Parser<'a, S> {
	fn new(stream: Stream<'a, S>) -> Self {
		Parser { stream }
	}
}

// impl<'a> Parser<'static, BufReader<Cursor<&'a str>>> {
// 	pub fn from_str(data: &'a str) -> Self {
// 		Stream::new(BufReader::new(Cursor::new(data)), None)
// 	}
// }

// impl<S:  Parser
// impl Parser<&'_ str> {
// 	pub fn new(s: &str) -> Self {
// 		let q = s.to_owned();
// 		Parser { 
// 			stream: Stream::from(q.as_str())
// 		}
// 	}
// }