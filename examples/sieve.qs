# A Hack until I get thread in the Kernel properly.
if(!__has_attr__($Thread), {
	Kernel.$Thread = { $x = spawn({}); $tmp = x.$__parents__.$get(0); x.$join(); tmp }();
});

# Creates an array from `_1` to `_2` and fills it with those numbers.
Number.$upto = {
	$arr = [];
	$start = _0.$clone();
	$stop = _1;

	while({ start <= stop }, {
		arr.$push(start.$clone());
		start += 1;
	});

	arr
};

$sieve = {
	$max = _1;
	$array = 2.$upto(max);

	2.$upto(max.$sqrt())
		.$map({
			$i = _0;

			if(!array.$get(i - 2), {
				return(:1);
			});

			spawn({
				$j = 0;
				while({ (:1.$k = i**2 + j*i) <= max }, {
					array.$set(k - 2, false);
					j += 1;
				});
			});
		})
		.$select(Number::$itself)
		.$each(Thread::$join);

	array.$select(Number::$itself)
};

$primes_upto_15 = sieve(15);
disp(primes_upto_15);

# Tests
assert(primes_upto_15 == [2, 3, 5, 7, 11, 13]);
