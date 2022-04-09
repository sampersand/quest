# `Comparable` is a mixin that's defined by default. It implements
# the `<`, `<=`, `>`, `>=` functions in terms of `<=>`


Person = object(Comparable) {
	'()' = (class, name, age) -> {
		:0.becomes(class)
	};

	'<=>' = (lhs, rhs) -> {
		lhs.age <=> rhs.age
	};
};

john = Person("john doe", 20);
jane = Person("jane doe", 22);

print(ifl(john > jane, john, jane).name, " is older");

# Tests
assert(john < jane);
assert("jane doe is older" == ifl(john > jane, john, jane).name + " is older");
