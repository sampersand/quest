"Person" = {
	"()" = {
		$__parent__ = _1;
		$first = _2;
		$last = _3;
		__this__
	};

	"@text" = {
		disp(__this__);
		(_1::'first') + " " + (_1::'last')
	};

	__this__
}();

disp(Person("Sam", "W"))