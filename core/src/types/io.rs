mod file;

pub use file::File;

use crate::{Object, Args, Literal};
use crate::types::Null;
use tracing::instrument;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Io;

impl Io {
	#[instrument(name="Io::each", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_each(this: &Object, args: Args) -> crate::Result<Object> {
		let (delim, block) = 
			if args.len() == 1 {
				("\n".into(), args.arg(0).unwrap())
			} else {
				(args.try_arg(0)?.clone(), args.try_arg(1)?)
			};

		loop {
			let line = this.call_attr_lit("read", &[&delim])?;

			if line.is_a::<Null>() {
				break
			} else {
				block.call_attr_lit(&Literal::CALL, &[&line])?;
			}
		}

		Ok(this.clone())
	}
}

impl_object_type!{
for Io [(parents super::Iterable)]:
	"File" => const file::File::mapping().clone(),
	"Stdin" => const file::File::from_fd(0, true, false).expect("cant create stdin"),
	"Stdout" => const file::File::from_fd(1, false, true).expect("cant create stdout"),
	"Stderr" => const file::File::from_fd(2, false, true).expect("cant create stdout"),
	"each" => method Self::qs_each,
}
