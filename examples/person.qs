# Create a person class. Classes are created by executing a block that returns
# `:0` (which is the same as `self`/`this in other languages).

Person = {
	# Define what it means to "call" a Person (ie `Person(...)`).
	# Because `() -> {...}` syntax already defines the arguments in the body
	# of the function, we don't need to set them ourselves. 
	'()' = (class, first, last) -> {
		# Set the current object's parents to `Person` so we can have access to
		# its `@text` method.
		__parents__ = [class];
		:0 # Return the current object, which we just created in this method.
	};

	# Define the conversion function to text.
	@text = person -> { person.first + ' ' + person.last };

	# As this is a class, we return `:0` at the end...
	:0
}(); # ... and immediately execute the block so as to create the class.

# Assign me as a new human.
sam = Person('Samp', 'Ersand');

# And greet me...
print("Hello, " + sam);


# Tests
assert(sam.first == 'Samp');
assert(sam.last == 'Ersand');
assert(sam.@text() == 'Samp Ersand');
