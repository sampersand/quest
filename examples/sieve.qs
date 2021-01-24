# A Hack until I get thread in the Kernel properly.
if(!:0.__has_attr__('Thread'), {
	Kernel.Thread = {
		x = spawn({});
		tmp = x.__parents__.get(0);
		x.join();
		tmp.spawn = Kernel::spawn;
		tmp
	}();
});

sieve = max -> {
	array = 2.upto(max);

	2.upto(max.sqrt())
		.map(i -> {
			array.get(i - 2).else(return);

			Thread::spawn {
				j = 0;
				while ({ (:1.k = i**2 + j*i) <= max }) {
					array[k - 2] = false;
					j += 1;
				}
			}
		})
		.each(~$join);

	array.select(~$@bool).@list()
};

primes_upto_15 = sieve(15);
print(primes_upto_15);

# Tests
assert(primes_upto_15 == [2, 3, 5, 7, 11, 13]);
