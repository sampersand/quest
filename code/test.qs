
$is_whitespace = {
	!if(_1.$get(1) != ' ', {
		if(_1.$get(1) != '\n', {
			if(_1.$get(1) != '\t', {
				true
			})
		})
	})
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

$parse_expr = {
	$line = lstrip(_1);
	$env = _2;
	$cmd = line.$shift();
	if(cmd, {
		if(('a' <= cmd) & (cmd <= 'z'), {
			{ env.cmd }
		}, {
			if(('0' <= cmd) & (cmd <= '9'), {
				{ cmd.$@num() }
			}, {
				known_opers.cmd(line, env)
			})
		})
	})
};

{
	$__this__ = __get_attr__($known_opers);
	'=' = { $l = lstrip(_1).$shift(); $r = parse_expr(_1, _2); $env = _2; { env.l = r() } };
	'+' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() + r() } };
	'-' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() - r() } };
	'*' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() * r() } };
	'/' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() / r() } };
	'|' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() | r() } };
	'&' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() & r() } };
	'<' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() < r() } };
	'>' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() > r() } };
	';' = { $l = parse_expr(_1, _2); $r = parse_expr(_1, _2); { l() ; r() } };
	'O' = { $a = parse_expr(_1, _2); { disp(a()) } };
	'P' = { $p = parse_expr(_1, _2); { prompt(p()) } };
	'Q' = { { quit() }};
	'R' = { $m = parse_expr(_1, _2); $x = parse_expr(_1, _2); { rand(m(), x()).$floor() } };
	'W' = { $c = parse_expr(_1, _2); $b = parse_expr(_1, _2); { while(c, b) } };
	'I' = { $c = parse_expr(_1, _2); $t = parse_expr(_1, _2); $f = parse_expr(_1, _2); { if(c(), t, f) } };
	'#' = {
		$nendl = true.$clone();
		$line = _1;
		while({ nendl & line },{
			if(line.$shift() == '\n', { nendl &= false; })
		});
		parse_expr(_1, _2)
	};
	'\'' = {
		$t = '';
		$line = _1;
		$not_found = true.$clone();
		while({ not_found & line }, {
			$c = line.$shift();
			if(c == '\'', {
				not_found &= false;
			}, {
				t += if(c == '\\', {
					$c = line.$shift();
					if(c == 'n', { '\n' }, { c })
				}, {
					c
				})
			})
		});
		{ t }
	};

	'(' = { parse_expr(_1, _2) };
	')' = { parse_expr(_1, _2) };
	__this__
}();

$knight = {
	$env = { $__parents__ = [Pristine]; __this__ }();
	env.null = null;
	parse_expr(_1, env)();
};

if(__has_attr__($_0), {
	if(_0 == '-e', {
		dispn(_1);
		knight(_1)
	}, {
		knight(system('cat', _0))
	})
}, {
	$t = '';
	$at_end=false;
	while({ !at_end },{
		$line=prompt();
		if(line, {
			t += line;
		}, {
			at_end |= true;
		});
	});
	knight(t);
});