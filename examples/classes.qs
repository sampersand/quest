# This is how you would represent classical "classes" within Quest by having
# the "class" be its own object too.

# Like in other examples, a "class" is really just an executed block of code
# which returns the local scope.
$Person = {
	# For `Scope`s, the `@text` attribute checks to see if a `name` field is set.
	# If we set one here, whenever we call `Person.$@text`, we'll get this name.
	$name = "Person";

	# There is no "constructor," per se. Generally, overloading the "call"
	# operator (i.e. `()`) is used to construct a class (by modifying the scope
	# of the function and returning `__this__` at the end), but this is just a
	# convention.
	$() = {
		# Since we're within a scope, `__parents__` defaults to `Scope`. We want to
		# change that so our parents is just `Person`. Note that `_0` is the object
		# that owns this method when it was called: This'll be `Parent`, or any
		# children subclassing it.
		$__parents__ = [_0.$instance_methods];

		# Assign local variables
		$first = _1;
		$last = _2;

		# We return the current scope, as it's the current object.
		__this__
	};

	$instance_methods = {
		# set parent to the calling stackframe
		$class = :1;

		$@text = {
			_0.$first + " " + _0.$last
		};

		__this__
	}();

	__this__
}();

$sam = Person("Sam", "W");

disp(Person); # => Person
disp(sam); # => Sam W
disp(sam.$class == Person); # => true

# Tests
assert(Person.$@text() == "Person");
assert(sam.$@text() == "Sam W");
assert(sam.$class == Person);
