Number.$square = { _1 ** 2 };
disp(4.$square());

Number.$factorial = {
	$__memo = { __this__.0 = (1); __this__ }();
	{
		disp(_2);
		if (!__memo.$__has_attr__(_1), {
			__memo._1 = ((_1 - 1).$factorial() * _1);
		})(_1);

		__memo._1
	}
}(1, 2);

12.$factorial()

# Kernel.$Frac = {
# 	$name = 'Frac';
# 	$() = {
# 		# parens are needed bc I don't have syntax parser perfectly done
# 		$__parents__ = [_1.$instance_methods];
# 		$numer = _2;
# 		$denom = _3;
# 		__this__
# 	};

# 	$instance_methods = {
# 		$class = _1;

# 		$+ = {
# 			$lhs = _1;
# 			$rhs = _2.$@num();
# 		# parens are needed bc I don't have syntax parser perfectly done
# 			Frac((lhs.$numer) + rhs * (lhs.$denom), lhs.$denom)
# 		};

# 		$@text = {
# 		# parens are needed bc I don't have syntax parser perfectly done
# 			'' + (_1.$numer) + '/' + (_1.$denom)
# 		};

# 		$@num = {
# 		# parens are needed bc I don't have syntax parser perfectly done
# 			(_1.$numer) / (_1.$denom)
# 		};

# 		__this__
# 	}(__this__);

# 	__this__
# }();

# $three_quarters = Frac(3, 4);
# disp(three_quarters + 4.3);