$Person = {
	$name = 'Person';

	$() = {
		$__parents__ = [_0.$ims];
		$first = _1;
		$last = _2;
		__this__
	};

	$ims = {
		$class = _0;

		$@text = {
			_0.$first + ' ' + _0.$last
		};

		__this__
	}(__this__);

	__this__
}();

$sam = Person("Sam", "W");
disp(sam);