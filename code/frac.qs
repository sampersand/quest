"and_and" = {
	$lhs = _1();
	$rhs = _2;
	if(lhs, rhs, {lhs})()
};

"Frac" = {
	"()" = {
		$__parent__ = _1;
		$numer = _2;
		$denom = _3;
		(__this__.'__del_attr__')('__args__');
		__this__
	};

	"@text" = {
		$__this__ = _1;
		('' + numer) + if(denom == 1, '', '/' + denom)
	};

	"@num" = {
		(_1.'numer') / (_1.'denom')
	};

	"to_frac" = {
		$obj = _1;
		if(and_and({(obj.'__has_attr__')('numer')}, {(obj.'__has_attr__')('denom')}), {
			obj
		}, {
			Frac(obj, 1)
		})()
	};

	"+" = {
		$rhs = (Frac::'to_frac')(_2);
		Frac(
			((_1.'numer') * (rhs.'denom')) + ((_1.'denom') * (rhs.'numer')),
			((_1.'denom') * (rhs.'denom')
		))
	};

	__this__
}();

$half = Frac(1, 2);
disp(3 ** half);

