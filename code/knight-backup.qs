$is_whitespace = {
	_1.$get(1) == ' ' || { _1.$get(1) == '\n' || { _1.$get(1) == '\t' } }
};

$lstrip = {
	if(is_whitespace(_1), {
		_1.$shift();
		lstrip(_1)
	}, {
		_1
	})
};

$known_opers = {__this__}();

Pristine.$|| = { if(_0, { _0 }, _1) };
Pristine.$&& = { if(_0, _1, { _0 }) };

$is_digit = { '0' <= _1 && { _1 <= '9'} };
$is_ident_start = { ('a' <= _1 && { _1 <= 'z' }) || { _1 == '_' } };
$is_cmd_start = { 'A' <= _1 && { _1 <= 'Z' } };
$is_ident = { is_ident_start(_1) || { is_digit(_1) } };
$is_cmd = { is_cmd_start(_1) || { _1 == '_' } };

$take_while = {
	$stream = _1;
	$fn = __get_attr__($_2);
	$acc = '';

	while($stream, {
		if(!fn(stream.$get(1)), {
			return(__stack__.$get(3), acc);
		});

		acc += stream.$shift();
	});

	acc
};

$next_ident = {
	$stream = _1;
	$ident = '';
	$fn = __get_attr__(if(__has_attr__($_2), { $_2 }, { $is_ident }));

	while($stream, {
		if(!fn(stream.$get(1)), {
			return(__stack__.$get(3), ident);
		});

		ident += stream.$shift();
	});

	ident
};


$next_number = { take_while(_1, __get_attr__($is_digit)) };

$next_text = {
	$text = '';
	$stream = _1;
	$not_found = true.$clone();
	while({ not_found & stream }, {
		$c = stream.$shift();
		if(c == '\'', {
			not_found &= false;
		}, {
			text += if(c == '\\', stream.$shift, { c });
		})
	});
	text
};

$parse_expr = {
	$stream = lstrip(_1);
	$chr = stream.$shift();

	if(!chr, {
		return(__stack__.$get(2));
	});

	if(is_ident_start(chr), {
		$ident = chr + next_ident(stream);
		return(__stack__.$get(2), { env.ident });
	});

	if(is_digit(chr), {
		$num = (chr + next_number(stream)).$@num();
		return(__stack__.$get(2), { num });
	});

	if(chr == '\'', {
		$text = next_text(stream);
		return(__stack__.$get(2), { text });
	});

	if(is_cmd_start(chr), {
		chr += next_ident(stream, __get_attr__($is_cmd));
	});

	known_opers.chr(stream, env)
};

{
	$__this__ = __get_attr__($known_opers);
	'=' = { $l = next_ident(lstrip(_1)); $r = parse_expr(_1, _2); { env.l = r() } };
	'+' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() + r() } };
	'-' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() - r() } };
	'*' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() * r() } };
	'/' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() / r() } };
	'|' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() | r() } };
	'&' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() & r() } };
	'<' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() < r() } };
	'>' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() > r() } };
	'^' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() **r() } };
	';' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() ; r() } };
	$O  = ($OUTPUT = { $a = parse_expr(_1, _2); { disp(a()) } });
	$P  = ($PROMPT = { $p = parse_expr(_1, _2); { prompt(p()) } });
	$Q  = ($QUIT = { quit });
	$R  = ($RAND ={ $m = parse_expr(_1, _2); $x = parse_expr(_1, _2); { rand(m(), x()).$floor() } });
	$W  = ($WHILE = { $c = parse_expr(_1, _2); $b = parse_expr(_1, _2); { while(c, b) } });
	$I  = ($IF = { $c = parse_expr(_1, _2); $t = parse_expr(_1, _2); $f = parse_expr(_1, _2); { if(c(), t, f) } });
	'#' = {
		$nendl = true.$clone();
		$stream = _1;
		while({ nendl & stream },{
			if(stream.$shift() == '\n', { nendl &= false; })
		});
		parse_expr(_1, _2)
	};
	$F = ($FNDEF = { $l = parse_expr(_1, _2); { __get_attr__($l) } });
	$S = ($SYSTEM = { $c = parse_expr(_1, _2); { system(c()) } });
	$C = ($CALL = { $l = parse_expr(_1, _2); { l()() } });
	$E = ($EVAL = { $w = parse_expr(_1, _2); { knight(w()) }});
	'(' = { parse_expr(_1, _2) };
	')' = { parse_expr(_1, _2) };
	__this__
}();

Kernel.$env = { $__parents__ = [Pristine]; __this__ }();
env.null = null;

$knight = {
	parse_expr(_1)()
};


knight(
	if(__has_attr__($_0), {
		if(_0 == '-e', { _1 }, { system('cat', _0) })
	}, {
		$t = '';
		while({ !at_end },{
			$stream=prompt();
			if(stream, {
				t += stream;
			}, {
				at_end |= true;
			});
		});
		t
	})
)