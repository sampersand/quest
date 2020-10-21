# Implementing the idea of inheritance in quest.
# this is done through manipulations of the the `__parents__` variable.
$Person = {
	$() = {
		$__parents__ = [_0];

		$first = _1;
		$last = _2;

		:0
	};

	$SAYS_WHAT = 'hi';
	$@text = { _0.$first + ' ' + _0.$last };
	$speak = { disp(_0, 'says:', _0.$SAYS_WHAT); };

	:0
}();

$Child = {
	$__parents__ = [Person];

	$SAYS_WHAT = "Waa! I want food!";

	$@text = {
		# this is bad, lol: todo builtin `super` function
		$parent_ims = _0.$__parents__.$get(0).$__parents__.$get(0);
		"Baby '" + parent_ims::$@text(_0) + "'"
	};

	:0
}();

$sam = Person('Sam', 'W');
$child = Child('Sammie', 'Boy');

sam.$speak(); # Sam W says: hi
child.$speak(); # Baby 'Sammie Boy' says: Waa! I want food!

# Tests
assert(sam.$@text() == "Sam W");
assert(sam.$SAYS_WHAT == "hi");

assert(child.$@text() == "Baby 'Sammie Boy'");
assert(child.$SAYS_WHAT == "Waa! I want food!");
