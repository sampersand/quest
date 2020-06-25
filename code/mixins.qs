$Enumerator = {
	$map = {
		$ret = [];
		$fn = _1;
		_0.$each({
			ret.$push(__get_attr__($fn)(_1))
		});
		ret
	};

	$select = {
		$ret = [];
		$fn = _1;
		_0.$each({
			if(__get_attr__($fn)(_1), {
				ret.$push(_1);
			});
		});

		ret
	};

	__this__
}();

$FunctionEnum = {
	__parents__.$push(Enumerator);

	$each = {
		$v = _0();
		if(v != null, {
			_1(v);
			_0.$each(__get_attr__($_1));
		})
	};
	__this__
}();

Number.$upto = {
	$i = _0;
	$MAX = _1;
	$ret = {
		$t = __this__;
		if(i < MAX, {
			$__this__ = t;
			__parents__.$get(2).$__parents__.$get(2).$i = i + 1
		})
	};
	__get_attr__($ret).$__parents__.$push(FunctionEnum);
	__get_attr__($ret)
};


List.$__parents__.$push(Enumerator);
List.$each = {
	$dup = _0.$clone();
	while({ dup }, {
		_1(dup.$shift())
	})
};

disp([1, 2, 3, 4, 5].$map({ _1 ** 2 }));
disp((-4).$upto(10)
		.$select({ _1 % 2 })
		.$map({ _1 ** 2 }));