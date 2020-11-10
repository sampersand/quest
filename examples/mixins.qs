# `Comparable` is a mixin that's defined by default. It implements
# the `<`, `<=`, `>`, `>=` functions in terms of `<=>`

Person = {
	__parents__.push(Comparable);
	
	'()' = (class, name, age) -> {__parents__ = [class]; :0 };

	'<=>' = (lhs, rhs) -> { lhs.age <=> rhs.age };

	:0
}();

john = Person("john doe", 20);
jane = Person("jane doe", 22);

disp(ifl(john > jane, john, jane).name, "is older");

# Tests
assert(john < jane);
assert("jane doe is older" == ifl(john > jane, john, jane).name + " is older");
