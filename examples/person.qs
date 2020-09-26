# A basic example of how a person "class" could be made.
$Person = {
	$() = {
		$__parents__ = [Person];
		$first = _1;
		$last = _2;
		__this__
	};

	$@text = {
		_0.$first + " " + _0.$last
	};

	__this__
}();

$sam = Person("Sam", "W");
disp(sam); # => Sam W


# Tests
assert(sam.$first == "Sam");
assert(sam.$last == "W");
assert(sam.$@text() == "Sam W");
