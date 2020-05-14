#![allow(unused)]

mod obj;
mod parse;

fn main() {
	// let x = [0xff]
	let mut stream = parse::Stream::from_str(r##"
		# a."+@"()
		# + += +@ - -= -@ * *= ** **= % %= / /= ! != = ==
		# < <= <=> << <<= > >= >> >>= ~ & &= && | |= || ^ ^= . .= .~ , ;
		#// (4 + (5 * 3)) * 3
#		(12.floor)(x);
# (12."floor")(x);
#((1 ** 2) * 3) + 4
{ (x), (y); (z) }
# "car" = { "x" = [_1, _2]."last"[]; (_1 * _2)."floor"(x) };

#		this.x = "123" + that.34; # this
#
#
#		foo = { _1 * (_2['3'] = _3) };
#
#		disp("hello there," + this.x);
	"##);
	let mut stream = stream.collect::<parse::Result<Vec<_>>>().unwrap().into_iter();

	println!("{:#?}", parse::Expression::try_from_iter(&mut stream));
}