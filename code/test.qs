$Person = {
	$name = "Person";

	$() = {
		$__parents__ = [_1.$instance_methods];
		disp(_1);

		$first = _2;
		$last = _3;

		__this__
	};

	$instance_methods = {
		$class = _1;

		$@text = {
			_1.$first + ' ' + _1.$last
		};

		$SAYS_WHAT = 'hi';

		$speak = {
			disp(_1, ':', _1.$SAYS_WHAT);
		};

		__this__
	}(__this__);

	__this__
}();

$Child = {
	$name = 'Child';
	$__parents__ = [Person];

	$instance_methods = {
		$__parents__ = [_1.$instance_methods];

		$class = _1;

		$@text = {
			"Baby " + _1.$__parents__.0.$@text(_1)
		};

		$SAYS_WHAT = "Waa! I want food!";

		__this__
	}(__this__);

	__this__
}();


$sam = Person('Sam', 'W');
$child = Child('Jace', 'B');
sam.$speak();
child.$speak();


##__EOF__##

#disp(sam.$instance_methods.$class);
#disp(sam.$FOO_BAR)

##__EOF_##



