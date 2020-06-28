# Implementing the idea of inheritance in quest.
# this is done through manipulations of the the `__parents__` variable.
$Person = {
	$() = {
		$__parents__ = [_0];

		$first = _1;
		$last = _2;

		__this__
	};

	$SAYS_WHAT = 'hi';
	$@text = { _0.$first + ' ' + _0.$last };
	$speak = { disp(_0, 'says:', _0.$SAYS_WHAT); };

	__this__
}();

$Child = {
	$__parents__ = [Person];

	$SAYS_WHAT = "Waa! I want food!";

	$@text = {
		# this is bad, lol: todo builtin `super` function
		$parent_ims = _0.$__parents__.$get(1).$__parents__.$get(1);
		"Baby '" + parent_ims::$@text(_0) + "'"
	};

	__this__
}();

$sam = Person('Sam', 'W');
$child = Child('Sammie', 'Boy');

sam.$speak(); # Sam W says: hi
child.$speak(); # Baby 'Sammie Boy' says: Waa! I want food!