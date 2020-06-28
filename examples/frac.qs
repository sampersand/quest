$Frac = {
	__parents__.$push(Comparable);

	$() = {
		$__parents__ = [Frac];
		$numer = _1;
		$denom = _2;
		__this__
	};

	$+ = {
		$lhs = _0;
		$rhs = _1.$@num();

		Frac(lhs.$numer + rhs * lhs.$denom, lhs.$denom)
	};

	$<=>   = { _0.$@num() <=> _1 };
	$@text = { _0.$numer.$@text() + '/' + _0.$denom };
	$@num  = { _0.$numer / _0.$denom };

	__this__
}();