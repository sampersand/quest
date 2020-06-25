$is_whitespace = {
	!if(_1.$get(1) != ' ', {
		if(_1.$get(1) != '\n', {
			if(_1.$get(1) != '\t', {
				true
			})()
		})()
	})()
};
$lstrip = {
	if(is_whitespace(_1), {
		_1.$shift();
		lstrip(_1)
	}, {
		_1
	})()
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
			})()
		})()
	})()
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
	'O' = { $a = parse_expr(_1, _2); { dispn(a()) } };
	'P' = { { prompt() } };
	'R' = { $m = parse_expr(_1, _2); $x = parse_expr(_1, _2); { rand(m(), x()).$floor() } };
	'W' = { $c = parse_expr(_1, _2); $b = parse_expr(_1, _2); { while(c, b) } };
	'I' = { $c = parse_expr(_1, _2); $t = parse_expr(_1, _2); $f = parse_expr(_1, _2); { if(c(), t, f)() } };
	'#' = {
		$endl = false.$clone();
		line = _1;
		while({ !endl & line },{
			line.$shift();
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
					if(c == 'n', '\n', c)
				}, {
					c
				})()
			})()
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
	parse_expr(_1, env)()
};

knight("
	; = m + 9 1
	; = s (R 1 m)
	; = n 0
	; O (+ (+ 'pick from 1-' m) '\\n')
	; = g 0
	; W (| (< g s) (> g s)) (
	  ; O '> '
	  ; = n (+ n 1)
	  ; = g (+ 0 P)
	  ; O (I (< g s)
	         'too low\\n'
	         I (> g s)
	            ('too high\\n')
	            (+ (+ 'perfect! it took you ' n) ' tries\\n')
	      )
");

##__EOF__##
knight("
; = m 10
; = s (R 1 m)
; = n 0
; O (+ (+ 'pick from 1-' m) '\n')
; = g 0
; W (| (< g s) (> g s)) ( # there's no `==` operator
  ; O '> '
  ; = n (+ n 1)
  ; = g (+ 0 P) # convert the prompt to an int
  ; O (I (< g s)
         ('too low\n')
         (I (> g s)
            ('too high\n')
            (+ (+ 'perfect! it took you ' n) ' tries\n')
         )
      )
");

##__EOF__##
$parse_ident = {
	$line = _1;
	$ident_not_found = true.$clone();
	$ident = '';
	while({ ident_not_found & line }, {
		$c = line.$shift();
		if((c == ' ') | (c == '\n'), {
			ident_not_found &= false;
		}, {
			ident.$push(c);
		})();
	});

	ident
};

$parse_expr = {
	$line = _1;
	$env = _2;
	env.$_line = line;
	$__this__ = env;
	_line.$eval()
};

$parse_assign = {
	$line = _1;
	$env = _2;
	$cmd = parse_ident(line);
	env.cmd = parse_expr(line, env)
};

$parse_if = {
	$line = _1;
	$env = _2;
};

$parse = {
	$line = _1;
	$env = _2;

	$op = line.$shift();
	if(op == '=', {
		env.$_ = parse_assign(line, env)
	}, {
		if(op == '?', {
			parse_if(line, env)
		}, {
			disp('unknown command: ' + op);
			quit();
		})()
	})()
};

$knight = {
	$text = _1;
	$env = {$_ = '\n'; __this__}();
	while($text, {
		$line = '';
		$not_newline = true.$clone();
		while({ not_newline & text }, {
			$c = text.$shift();
			if(c == '\n', {
				not_newline &= false;
			}, {
				line.$push(c);
			})()
		});

		parse(line, env)
	});
	env.$_
};

disp(knight("\
=x 0
W + 3 4 (12) disp(13)
"));

##__EOF__##
$calc_rpn = {
	$stack = [];
	$rpn = _1;
	$curr = '';
	while($rpn, {
		$c = rpn.$shift();
		if(('0' <= c) & (c <= '9') | (c == '.'), {
			curr.$push(c);
		}, {
			if(curr, {
				stack.$push(curr.$@num());
				curr.$clear();
			})();

			if((c == '+') | (c == '-') | (c == '*') | (c == '/'), {
				$rhs = stack.$pop();
				$lhs = stack.$pop();
				stack.$push(lhs.c(rhs));
			})();
		})();
	});

	stack.$pop()
};

disp(calc_rpn("12 24 - 12.4 *"));