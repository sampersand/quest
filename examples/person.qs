# A basic example of how a person "class" could be made.
$Person = {
	$() = {
		$__parents__ = [Person];
		$first = _1;
		$last = _2;
		:0 # return the current stackframe
	};

	$@text = {
		_0.$first + " " + _0.$last
	};

	:0 # return the current stackframe
}();

$sam = Person("Sam", "W");
disp(sam); # => Sam W