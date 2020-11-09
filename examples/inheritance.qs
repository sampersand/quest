# Implementing the idea of inheritance in quest.
# this is done through manipulations of the the `__parents__` variable.
Person = {
	'()' = (class, first, last) -> { __parents__ = [class]; :0 };

	SAYS_WHAT = 'hi';

	@text = person -> {
		person.first + ' ' + person.last
	};

	speak = person -> {
		disp(person, 'says:', person.SAYS_WHAT);
	};

	:0
}();

Child = {
	__parents__ = [Person];

	SAYS_WHAT = "Waa! I want food!";

	@text = child -> {
		# This is a hack until I get the `super` function implemented.
		parent_ims = child.__parents__.get(0).__parents__.$get(0);
		"Baby '" + parent_ims::$@text(child) + "'"
	};

	:0
}();

$sam = Person('Sam', 'W');
$child = Child('Sammie', 'Boy');

sam.$speak(); # Sam W says: hi
child.$speak(); # Baby 'Sammie Boy' says: Waa! I want food!

# Tests
assert(sam.@text() == "Sam W");
assert(sam.SAYS_WHAT == "hi");

assert(child.@text() == "Baby 'Sammie Boy'");
assert(child.SAYS_WHAT == "Waa! I want food!");
