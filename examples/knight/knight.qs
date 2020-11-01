Text.take_while = (self, fn) -> {
	acc = '';

	while(fn << self, {
		acc += self.shift();
	});

	acc
};

Text.next_ident = self -> {
	/^[a-z_]/.match(self).then(self.take_while << /^[a-z\d_]/.match)
};

Text.next_cmd = self -> {
	/^[A-Z]/.match(self).then(self.take_while << /^[A-Z_\d]/.match)
};

Text.next_num = self -> {
	/^\d/.match(self).then({ self.take_while(/^\d/.match).@num() })
};

Text.next_text = self -> {
	/^['"]/.match(self).then({
		quote = self.shift();
		l = self.take_while({ self.then({ self.get(0) != quote }) });
		self.shift();
		l
	})
};

knight = func -> { func.next_expr()() };

functions = { :0 }();

{
	:0 = functions;
	unary_fn = op -> {
		stream -> {
			x = stream.next_expr();
			{ op(x()) }
		}
	};

	binary_fn = op -> {
		stream -> {
			l = stream.next_expr();
			r = stream.next_expr();
			{ op(l(), r()) }
		}
	};

	'!' = unary_fn(Basic::'!');
	Q = QUIT = unary_fn(quit);
	O = OUTPUT = unary_fn(disp);
	P = PROMPT = unary_fn(prompt);
	E = EVAL = unary_fn(knight);
	S = SYSTEM = unary_fn(system); # this doesn't work currently
	F = FNDEF = stream -> { l = stream.next_expr(); l.itself };
	C = CALL = unary_fn({ _0() });

	'+' = binary_fn(Number::'+');
	'-' = binary_fn(Number::'-');
	'*' = binary_fn(Number::'*');
	'/' = binary_fn(Number::'/');
	'^' = binary_fn(Number::'**');
	'%' = binary_fn(Number::'%');
	'&' = binary_fn(Number::'&');
	'|' = binary_fn(Number::'|');
	'<' = binary_fn(Number::'<');
	'>' = binary_fn(Number::'>');
	';'= binary_fn({ _1 });
	'=' = stream -> {
		stream.replace(stream.strip());
		var = stream.next_ident();
		val = stream.next_expr();

		{ env.(var) = val }
	};
	R = RAND = binary_fn({ rand(_0, _1).round() });
	W = WHILE = { while << (_0.next_expr()) << (_0.next_expr()) };

	I = IF = {
		c = _0.next_expr();
		c.@bool = { _0().@bool() };

		if << c << (_0.next_expr()) << (_0.next_expr())
	};
	:0
}();

Text.next_expr = self -> {
	self.replace(self.strip());

	while(/^#/.match << self, {
		self.take_while(/^\n/.match);
		self.replace(self.strip());
	});

	self.else(return);

	(null != (ident = self.next_ident())).then({ env.(ident) }.return);
	(null != (num = self.next_num())).then(num.itself.return);
	(null != (text = self.next_text())).then(text.itself.return);

	cmd = self.next_cmd().else(self.shift);

	(functions::(cmd))(self)
};

Kernel.env = { :0.null = null; :0 }();

if(:0.__has_attr__('_1'), {
	if(_1 == '-e', {
		knight(_2)
	}, {
		knight(system('cat', _1))
	})
}, {
	t = '';
	at_end=false;
	while({ !at_end },{
		stream=prompt();
		if(stream, {
			t += stream;
		}, {
			at_end |= true;
		});
	});
	knight(t);
});
