use crate::{Object, Args, Result};
use crate::types::Text;

use std::sync::{Arc, Mutex};
use std::net::{ToSocketAddrs, TcpStream};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Tcp(Arc<Mutex<TcpStream>>);

impl Tcp {
	#[inline]
	pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
		Ok(Tcp(Arc::new(Mutex::new(TcpStream::connect(addr)?))))
	}
}

impl Tcp {
	pub fn qs_call(_: &Object, args: Args) -> Result<Object> {
		args.arg(0)?.try_downcast_and_then(|addr: &Text| {
			Tcp::connect(addr.as_ref())
				.map(Object::from)
				.map_err(|err| crate::Error::Messaged(err.to_string()))
		})
	}
}

impl_object_type!{
for Tcp [(parents super::Basic)]:
	"()" => function Tcp::qs_call,
	"write" => function |this: &Object, args: Args| {
		let arg = args.arg(0)?.call_downcast_map(Text::clone)?;

		this.try_downcast_mut_and_then(|tcp: &mut Tcp| {
			tcp.0.lock().unwrap().write(&arg.as_ref().as_ref())
				.map(Object::from)
				.map_err(|err| crate::Error::Messaged(err.to_string()))
		})
	},
	"read" => function |this: &Object, _: Args| -> Result<Object> {
		this.try_downcast_mut_and_then::<_, _, crate::Error, _>(|tcp: &mut Tcp| {
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














