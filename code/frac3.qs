Kernel.$Frac = {
	$name = 'Frac';
	$() = {
		# parens are needed bc I don't have syntax parser perfectly done
		$__parent__ = (_1.$instance_methods);
		$numer = _2;
		$denom = _3;
		__this__
	};

	$instance_methods = {
		$class = _1;

		$+ = {
			$lhs = _1;
			$rhs = _2.$@num();
		# parens are needed bc I don't have syntax parser perfectly done
			Frac((lhs.$numer) + rhs * (lhs.$denom), lhs.$denom)
		};

		$@text = {
		# parens are needed bc I don't have syntax parser perfectly done
			'' + (_1.$numer) + '/' + (_1.$denom)
		};

		$@num = {
		# parens are needed bc I don't have syntax parser perfectly done
			(_1.$numer) / (_1.$denom)
		};

		__this__
	}(__this__);

	__this__
}();

$three_quarters = Frac(3, 4);
disp(three_quarters + 4.3);