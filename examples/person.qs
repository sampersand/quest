# Create a person class. Classes are created by executing a block that returns
# `__this__`. (Note: $FOO is identical to "FOO"---it's just syntactic sugar)
$Person = {

	# Define what it means to "call" a Person (ie `Person(...)`)
	$() = {

		# Set the current object's parents to `Person` so we can have access to
		# its `@text` method.
		$__parents__ = [Person];
		
		# Assign first name to the first argument and last name to the second.
		$first = _1;
		$last = _2;

		__this__ # Return the current object, which we just created in this method.
	};

	# Define the conversion function to text
	$@text = {
		_0.$first + ' ' + _0.$last
	};

	# As this is a class, we return `__this__` at the end...
	__this__
}(); # ... and immediately execute the block so as to create the class.

# Assign me as a new human.
$sam = Person('Sam', 'W');

# And greet me...
disp("Hello, " + sam);


# Tests
assert(sam.$first == "Sam");
assert(sam.$last == "W");
assert(sam.$@text() == "Sam W");
