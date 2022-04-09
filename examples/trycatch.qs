trycatch = (try, catch) -> {
	__try__ = true;
	value = try();
	if(__try__, { value }, { catch(value) })
};

throw = exception -> {
	i = 0;
	frame = _0;

	while ({ !frame.__keys__(false).include?('__try__') }) {
		i += 1;
		:1.frame = __stack__[i];
	};

	frame.__try__ = false;
	exception.return(__stack__[i - 2]);
};

handler = block -> {
	print(trycatch(block) { "Exception: " + _0 });
};

handler() {
	return("Hello, world!");
	assert(false);
};

handler() {
	throw("Hello, exception!");
	assert(false);
};

