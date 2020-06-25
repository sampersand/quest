Number.'square' = { _1 ** 2 };
disp( 4.'square'() ); #=> 16


	# `_1` is the first argument, ie "this"/"self"

Number.$fac = {
	$__memo = { __this__.0 = 1; __this__ }();

	{
		# we can't take fac of non-positive ints.
		# $_1 = (_1.$floor().$abs());

		# if the memo doesn't contain us
		if(!__memo.$__has_attr__($_1 = (_1.$floor().$abs())), {
			disp('adding', _1, 'to __memo');

			# assign us to it.
			__memo._1 = ((_1 - 1).$fac() * _1);
		});

		# return the resulting value in the memo
		__memo._1
	}
}();


disp('5! =', 5.3.$fac());
disp('10! = ', 10.$fac());













