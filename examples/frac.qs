$Frac = {
	__parents__.$push(Comparable);

	$() = {
		$__parents__ = [Frac];
		$numer = _1;
		$denom = _2;
		:0
	};

	$+ = {
		$lhs = _0;
		$rhs = _1.$@num();

		Frac(lhs.$numer + rhs * lhs.$denom, lhs.$denom)
	};

	$<=>   = { _0.$@num() <=> _1 };
	$@text = { _0.$numer.$@text() + '/' + _0.$denom };
	$@num  = { _0.$numer / _0.$denom };

	:0
}();

# Tests
assert(Frac(3, 4).$@text() == "3/4");
assert(Frac(1, 2) + Frac(3, 4) == Frac(5, 4));
assert(Frac(1, 2) < Frac(3, 4));
assert(Frac(1, 2).$@num() == 0.5);
