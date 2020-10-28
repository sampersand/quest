Text.$take_while = {
	$acc = '';

	while(_1 << _0, {
		acc += _0.$shift();
	});

	acc
};


Text.$next_ident = {
	/^[a-z_]/.$match(_0).$then(_0.$take_while << /^[a-z\d_]/.$match)
};

Text.$next_cmd = {
	/^[A-Z]/.$match(_0).$then(_0.$take_while << /^[A-Z_\d]/.$match)
};

Text.$next_num = {
	/^\d/.$match(_0).$then({ _0.$take_while(/^\d/.$match).$@num() })
};

Text.$next_text = {
	/^['"]/.$match(_0).$then({
		$quote = _0.$shift();
		$l = _0.$take_while({ _0.$then({ _0.$get(0) != quote }) });
		_0.$shift();
		l
	})
};

$knight = { _0.$next_expr()() };

$functions = {:0}();

{
	:0 = functions;
	$unary_fn = {
		$op = _0;
		{
			$x = _0.$next_expr();
			{ op(x()) }
		}
	};

	$binary_fn = {
		$op = _0;
		{
			$l = _0.$next_expr();
			$r = _0.$next_expr();
			{ op(l(), r()) }
		}
	};

	$! = unary_fn(Basic::$!);
	$Q = $QUIT = unary_fn(quit);
	$O = $OUTPUT = unary_fn(disp);
	$P = $PROMPT = unary_fn(prompt);
	$E = $EVAL = unary_fn(knight);
	$S = $SYSTEM = unary_fn(system); # this doesn't work currently
	$F = $FNDEF = { $l = _0.$next_expr(); l.$itself };
	$C = $CALL = unary_fn({ _0() });

	$+ = binary_fn(Number::$+);
	$- = binary_fn(Number::$-);
	$* = binary_fn(Number::$*);
	$/ = binary_fn(Number::$/);
	$^ = binary_fn(Number::$**);
	$% = binary_fn(Number::$%);
	$& = binary_fn(Number::$&);
	$| = binary_fn(Number::$|);
	$< = binary_fn(Number::$<);
	$> = binary_fn(Number::$>);
	';'= binary_fn({ _1 });
	$= = {
		_0.$replace(_0.$strip());
		$l = _0.$next_ident();
		$r = _0.$next_expr();
		{ env.l = r() }
	};
	$R = $RAND = binary_fn({ rand(_0, _1).$round() });
	$W = $WHILE = { while << (_0.$next_expr()) << (_0.$next_expr()) };

	$I = $IF = {
		$c = _0.$next_expr();
		c.$@bool = { _0().$@bool() };

		if << c << (_0.$next_expr()) << (_0.$next_expr())
	};
	:0
}();

Text.$next_expr = {
	_0.$replace(_0.$strip());

	while(/^#/.$match << _0, {
		_0.$take_while(/^\n/.$match);
		_0.$replace(_0.$strip());
	});

	_0.$else(return);

	(null != ($ident = _0.$next_ident())).$then(ident.$return);
	(null != ($num = _0.$next_num())).$then(num.$return);
	(null != ($text = _0.$next_text())).$then(text.$return);

	$cmd = _0.$next_cmd().$else(_0.$shift);

	(functions::cmd)(_0)
};

Kernel.$env = { :0.null = null; :0 }();

knight('O 319 1 12');
quit();
if(:0.$__has_attr__($_1), {
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
