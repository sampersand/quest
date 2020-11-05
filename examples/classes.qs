# This is how you would represent classical "classes" within Quest by having
# the "class" be its own object too.
#
# Like in other examples, a "class" is really just an executed block of code
# which returns the local scope.
Person = {
	# For `Scope`s, the `@text` attribute checks to see if a `name` field is set.
	# If we set one here, whenever we call `Person.@text`, we'll get this name.
	name = "Person";

	# There is no "constructor," per se. Generally, overloading the "call"
	# operator (i.e. `()`) is used to construct a class (by modifying the scope
	# of the function and returning `:0` (which means `self`/`this` in other
	# languages) at the end), but this is just a convention.
	'()' = (class, first, last) -> {
		# Since we're within a scope, `__parents__` defaults to `Scope`. We want to
		# change that so our parents is just `Person`'s instance methods.
		__parents__ = [class.instance_methods];

		# We return the current scope, as it's the current object.
		:0
	};

	instance_methods = {
		@text = person -> {
			person.first + " " + person.last
		};

		:0
	}();

	:0
}();

samp = Person("Samp", "Ersand");

disp(Person); # => Person
disp(samp); # => Samp Ersand
disp(samp.class == Person); # => true

# Tests
assert(Person.@text() == "Person");
assert(samp.@text() == "Samp Ersand");
assert(samp.class == Person);
