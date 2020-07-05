# This program I use to benchmark how fast quest is becoming!
# When I first started, `50,000` times took 176 seconds... lol.
# At last run, this took ~1.6 seconds to run. Not shabby, but not great either

$fib_loop = {
	if(!_1, { return(:1, 0) });

	$a = 0;
	$b = 1;
	$_1 = _1.$clone(); # clone it because we end up modifying it with `-=`

	while({ _1 -= 1 }, {
		:1.$b = ($t = b) + a;
		:1.$a = t;
	});

	b
};

$_1 = if(__has_attr__($_1), { _1.$@num() }, { 50_000 });


disp(fib_loop(_1));