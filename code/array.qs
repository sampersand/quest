Kernel.$for = {
	$i = _1;
	while({ _2([i] + __args__[4, -1]) }, {
		$res = _3([i] + __args__[4, -1]);
		_1.'i' = i + 1;
		res
	}, [__this__] + __args__[2, -1])
};

$Array = {
	$__parent__ = List;
	$name = 'Array';
	$() = {
		$__parent__ = _1::'instance_methods';

		for(1, { _1 != (_3.'len')() }, {
			_2._1 = _3[_1 + 1];
		}, __this__, __args__);

		$len = (__args__.'len')() - 1;

		(__this__.'__del_attr__')('__args__');

		__this__
	};

	$instance_methods = {
		$class = _1;

		$@list = {
			$__parent__ = _1;
			$list = [];
			for(1, { 1 != _1 <=> len }, { list << _2._1 }, __this__)
		};

		"+" = {
			_1((_1.'@list')() + (_2.'@list')())
		};

		$@text = {
			$__this__ = _1;
			((__this__.'@list')().'@text')()
		};

		$[] = { _1._2 };

		__this__
	}(__this__);
	__this__
}();

$arr = Array('a','b','c','d');
disp(arr + [2,3]);



