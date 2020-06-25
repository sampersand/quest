Kernel.$factorial = {
	$memo = {$__parent__ = Pristine; __this__}();
	{
		$__parent__ = _0;
		$n = _1;

		if((memo.'__has_attr__')(n), {
			_0.'memo'.(_0.'n')
		}, {
			$__this__ = _0;
			memo.n = if(n < 2, { 1 }, { n * factorial(n - 1) })
		})
	}
}();

factorial(3)
# factorial(30)