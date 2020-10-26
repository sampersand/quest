use crate::{Object, Args, Result};
use crate::types::Text;

use std::sync::{Arc, Mutex};
use std::net::{ToSocketAddrs, TcpStream};
use std::io::{self, Write};
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Tcp(Arc<Mutex<TcpStream>>);

impl Tcp {
	#[inline]
	pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
		Ok(Tcp(Arc::new(Mutex::new(TcpStream::connect(addr)?))))
	}
}

impl Tcp {
	#[instrument(name="Tcp::()", level="trace")]
	pub fn qs_call(_: &Object, args: Args) -> Result<Object> {
		args.try_arg(0)?.try_downcast::<Text>().and_then(|addr| {
			Tcp::connect(addr.as_ref())
				.map(Object::from)
				.map_err(|err| crate::Error::Messaged(err.to_string()))
		})
	}
}

impl_object_type!{
for Tcp [(parents super::Basic)]:
	"()" => method Self::qs_call,
	"get" => method |this, _| {
		Ok(ureq::get(this.call_downcast::<Text>()?.as_ref())
			.call()
			.into_string()
			.unwrap()
			.into())
	},
	"write" => method |this, args| {
		let arg = args.try_arg(0)?.call_downcast::<Text>()?.clone();

		this.try_downcast_mut::<Self>().and_then(|tcp| {
			tcp.0.lock().unwrap().write(&arg.as_ref().as_ref())
				.map(Object::from)
				.map_err(|err| crate::Error::Messaged(err.to_string()))
		})
	},
	"read" => method |this, _| -> Result<Object> {
		this.try_downcast_mut::<Self>().and_then(|tcp| {
			use std::io::{BufReader, BufRead};
			let mut res = Vec::<u8>::with_capacity(5);
			let tcp = tcp.0.lock().unwrap();
			let mut bufr = BufReader::new(&*tcp);
			while bufr.read_until(b'\n', res.as_mut())
				.map_err(|err| crate::Error::Messaged(err.to_string()))? != 0
			{
				if res.ends_with(b"\r\n\r\n") {
					break;
				}
			}

			Ok(String::from_utf8_lossy(&res).into_owned().into())
		})
	},
}
