$Frac = {
	$name = 'Frac';

	$() = {
		$__parents__ = [_1.$ims];
		$numer = _2;
		$denom = _3;
		__this__
	};

	$ims = {
		$class = _1;
		__parents__.$push Comparable;

		$+ = {
			$lhs = _1;
			$rhs = _2.$@num();
			Frac(lhs.$numer + rhs * lhs.$denom, lhs.$denom)
		};

		$<=>   = { _1.$@num() <=> _2 };
		$@text = { _1.$numer.$@text() + '/' + _1.$denom };
		$@num  = { _1.$numer / _1.$denom };

		__this__
	}(__this__);

	__this__
}();
__this__