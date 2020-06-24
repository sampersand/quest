$x = ($y = 1);
disp(x, y)
###__EOF__##
$fib = {
	$memo = { __this__ }();
	memo.0 = 1;
	memo.1 = 1;
	{
		if(memo.$__has_attr__(_2), {
			memo._2
		}, {
			memo._2 = fib(_2 - 1) + fib(_2 - 2);
			memo._2
		})()
	}
}();

disp(fib(10));
##__EOF__##
time ruby -e '$m={0=>1,1=>1};def fib(x)$m[x]||=fib(x-1)+fib(x-2)end;fib(50)'


time ./target/debug/quest_exec -e '$fib={
	$m={__this__}();
	m.0=m.1=1;
	{if(m.$__has_attr__(_2),{m._2},{m._2=fib(_2-1)+fib(_2-2); m._2})()}
}(); disp(fib(50))'