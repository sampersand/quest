# `Comparable` is a mixin that's defined by default. It implements
# the `<`, `<=`, `>`, `>=` functions in terms of `<=>`

$Person = {
	__parents__.$push(Comparable);

	$() = {
		$__parents__ = [Person];

		$name = _1;
		$age = _2;

		:0 # return the current stackframe
	};

	$<=> = { _0.$age <=> _1.$age };

	:0 # return the current stackframe
}();

$john = Person("john doe", 20);
$jane = Person("jane doe", 22);
disp(john < jane); # => true