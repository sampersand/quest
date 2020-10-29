Frac = {
	__parents__.push(Comparable);

	'()' = (class, numer, denom) -> {
		if(denom == 0, return); # return `null` if the denom is zero.

		__parents__ = [class];
		:0
	};

	# Checks to see if the fraction is a whole number.
	is_whole = frac -> {
		frac.denom == 1
	};

	# Add any number to `lhs`, returning a new Frac.
	'+' = (lhs, rhs) -> {
		Frac(lhs.numer + lhs.denom * rhs, lhs.denom)
	};

	# Compare a Frac with something else by converting both to a number.
	'<=>' = (lhs, rhs) -> {
		lhs.@num() <=> rhs.@num()
	};

	# Convert a Frac to a Boolean by checking to see if it's not zero.
	@bool = this -> {
		this.numer != 0
	};

	# Convert a Frac to a Number by simply dividing the `numer` by the `denom`.
	@num = this -> { this.numer / this.denom };

	# Convert a Frac to a Text by returning `numer/denom`, omitting `/denom` if
	# we're a whole number.
	@text = this -> {
		numertxt = this.numer.@text();
		this.is_whole().then(numertxt.return);
		numertxt + '/' + this.denom
	};

	:0
}();

# Tests
assert(Frac(3, 4).@text() == "3/4");
assert(Frac(3, 1).@text() == "3");
assert(Frac(1, 2) + Frac(3, 4) == Frac(5, 4));
assert(Frac(1, 2) < Frac(3, 4));
assert(Frac(1, 2).@num() == 0.5);
assert(Frac(1, 2).@bool() == true);
assert(Frac(0, 2).@bool() == false);
assert(Frac(12, 0) == null);
