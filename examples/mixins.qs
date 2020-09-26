# `Comparable` is a mixin that's defined by default. It implements
# the `<`, `<=`, `>`, `>=` functions in terms of `<=>`

$Person = {
	__parents__.$push(Comparable);
	
	$() = {
		$__parents__ = [Person];

		$name = _1;
		$age = _2;

		__this__
	};

	$<=> = { _0.$age <=> _1.$age };

	__this__
}();

$john = Person("john doe", 20);
$jane = Person("jane doe", 22);

disp(if(john > jane, $john, $jane).$name, "is older");

# Tests
assert(john < jane);
assert("jane doe is older" == if(john > jane, $john, $jane).$name + " is older");
