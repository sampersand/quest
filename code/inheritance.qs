$Person = {
	$name = "Person";

	$() = {
		$__parents__ = [_0.$instance_methods];

		$first = _1;
		$last = _2;

		__this__
	};

	$is_parent = true;

	$instance_methods = {
		$class = _0;

		$@text = {
			_0.$first + ' ' + _0.$last
		};

		$SAYS_WHAT = 'hi';

		$speak = {
			disp(_0.$@text() + ':', _0.$SAYS_WHAT);
		};

		__this__
	}(__this__);

	__this__
}();

$Child = {
	$__parents__ = [Person];

	$is_parent = false;

	$instance_methods = {
		$class = _0;

		$__parents__ = [_0::$instance_methods];

		$SAYS_WHAT = "Waa! I want food!";

		$@text = {
			# this is bad, lol: todo builtin `super` function
			$parent_ims = _0.$__parents__.$get(1).$__parents__.$get(1);
			"Baby " + parent_ims::$@text(_0)
		};

		__this__
	}(__this__);

	__this__
}();

$sam = Person('Sam', 'W');
$child = Child('Sammie', 'Hammie');
sam.$speak();
child.$speak();
disp(sam.$is_parent)


