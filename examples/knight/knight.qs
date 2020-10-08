Pristine.'||' = { if(_0, $_0, _1) };
Pristine.'&&' = { if(_0, _1, $_0) };

Text.$lstrip = {
	while(/^[\s()]/.$match << _0, _0.$shift);
	_0
};

Text.$next_ident = {
	if(/^[a-z_]/.$match(_0), _0.$take_while << /^[a-z\d_]/.$match)
};

Text.$next_cmd = {
	if(/^[A-Z]/.$match(_0), _0.$take_while << /^[A-Z_\d]/.$match)
};

Text.$next_num = {
	if(/^\d/.$match(_0), { _0.$take_while(/^\d/.$match).$@num() })
};

Text.$next_text = {
	if(_0.$get(0) == "'", {
		_0.$shift();
		_0.$take_while({ ($x=_0) && { x.$get(0) != "'" } })
	})
};

Text.$take_while = {
	$acc = '';

	while(:0::$_1 << _0, {
		acc += _0.$shift();
	});

	acc
};

$functions = {
	$unary_fn = {
		$op = :0::$_1;
		{
			$x = _0.$next_expr();
			{ :0::$op(x()) }
		}
	};

	$binary_fn = {
		$op = :0::$_1;
		{
			$l = _0.$next_expr();
			$r = _0.$next_expr();
			{ :0::$op(l(), r()) }
		}
	};

	$! = unary_fn(Basic::$!);
	$Q = ($QUIT = unary_fn(quit));
	$O = ($OUTPUT = unary_fn(disp));
	$P = ($PROMPT = unary_fn(prompt));
	$E = ($EVAL = unary_fn({ knight(_0) }));
	$S = ($SYSTEM = unary_fn(system)); # this doesn't work currently
	$F = ($FNDEF = { $l = _0.$next_expr(); { l } });
	$C = ($CALL = unary_fn({ _0() }));

	$+ = binary_fn(Number::$+);
	$- = binary_fn(Number::$-);
	$* = binary_fn(Number::$*);
	$/ = binary_fn(Number::$/);
	$^ = binary_fn(Number::$**);
	$& = binary_fn(Number::$&);
	$| = binary_fn(Number::$|);
	$< = binary_fn(Number::$<);
	$> = binary_fn(Number::$>);
	';'= binary_fn({ _1 });
	$= = {
		_0.$lstrip();
		$l = _0.$next_ident();
		$r = _0.$next_expr();
		{ env.l = r() }
	};
	$R = ($RAND = binary_fn({ rand(_0, _1).$round() }));
	$W = ($WHILE = { while << (_0.$next_expr()) << (_0.$next_expr()) });

	$I = ($IF = {
		$c = _0.$next_expr();
		:0::$c.$@bool = { _0().$@bool() };

		if << (:0::$c) << (_0.$next_expr()) << (_0.$next_expr())
	});

	:0
}();


Text.$next_expr = {
	_0.$lstrip();

	while({ /^#/.$match(_0) }, {
		_0.$take_while({ ($v=_0.$get(0)) != null && { v.$get(0) != "\n" } });
		_0.$lstrip();
	});

	if(!_0, { return(:1); });

	if(null != ($ident = _0.$next_ident()), {
		return(:1, { env.ident });
	});

	if(null != ($num = _0.$next_num()), {
		return(:1, { num });
	});

	if(null != ($text = _0.$next_text()), {
		return(:1, { text });
	});

	$cmd = _0.$next_cmd() || _0.$shift;

	(functions::cmd)(_0)
};

Kernel.$env = {:0.null = null; :0}();

$knight = {
	_1.$next_expr()()
};

if(__has_attr__($_1), {
	if(_1 == '-e', {
		knight(_2)
	}, {
		knight(system('cat', _1))
	})
}, {
	$t = '';
	$at_end=false;
	while({ !at_end },{
		$stream=prompt();
		if(stream, {
			t += stream;
		}, {
			at_end |= true;
		});
	});
	knight(t);
});
