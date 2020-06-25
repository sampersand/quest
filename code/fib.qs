$fib = {
	$memo = { __this__ }();
	memo.0 = 1;
	memo.1 = 1;
	{
		if(memo.$__has_attr__(_1), {
			memo._1
		}, {
			memo._1 = fib(_1 - 1) + fib(_1 - 2);
			memo._1
		})
	}
}();

disp(fib(50));