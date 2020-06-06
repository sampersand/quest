$literal_array = ['hello', 'world'];

disp(literal_array[1], literal_array[2]);

$array = {
	__this__.1 = 'hello';
	__this__.2 = 'world';
	__this__
}();

disp(array.1, array.2);

$map = {
	$greeting = "hello";
	$location = "world";
	__this__
}();

disp(map.$greeting, map.$location);

$Greater = {
	$name = "Greater";
	$() = {
		$__parent__ = (_1.$instance_methods);
		$greeting = _2;
		$location = _3;
		__this__
	};

	$instance_methods = {
		$class = _1;

		$greet = {
			disp(_1.$greeting, _1.$location);
		};

		__this__
	}(__this__);

	__this__
}();

Greater("hello", "world").$greet();