# A class is just an executed block of code
$Person = {
	# Set the name of this class to `Person`. Executed blocks automatically inherit from
	# `ExecutedBlock`, which defines `@text` as `name` (if a name is set)
	$name = "Person";

	# The "constructor"; to make a new person, you do `Person(...)`, which calls this function
	$() = {
		# The first argument to a function is always object the function's bound to. In this case,
		# the first argument (_1) is Person.
		# 
		# We don't want the child to be a direct descendant of Person, then we'd have `name` defined.
		# Instead, have it inherit from the Person's instance methods.
		$__parent__ = _1::$instance_methods ;

		# assign the first and last names to the second and third parameters
		$first = _2;
		$last = _3;

		# By returning `this`, we return the current scope we're in. All that's defined is 
		# `__parent__`, `first` and `last`---exactly what classes in other languages have!
		__this__
	};

	# Define the instance methods of the class.
	$instance_methods = {
		# This is so we can access the `Parent` class.
		$class = _1;

		# We define the `@text` method for a person by combining their first and last names
		$@text = {
			(_1.$first ) + ' ' + _1.$last
		};

		# define what it means for a person to speak
		$says_what = 'hi!';

		$speak = {
			# speaking is `<name>: <message>`
			_1.$@text() + ': ' + _1.$says_what
		};

		# Return this as the last argument, so we can use it as a parent.
		__this__
	}(__this__); # pass __this__ as the first argument

	__this__
}();

# Now we make a child that inherits from parent
$Child = {
	# All the things parent defines we now have
	$__parent__ = Person;

	$name = "Child";

	# A child will have its own instance methods.
	$instance_methods = {
		# We want these instance methods to inherit from the Parent's instance methods
		$__parent__ = _1::$instance_methods;

		# Redefine what it means to convert a child to text.
		$@text = {
			# Get the `super` method. `_1.$__parent__` is simply the child's instance methods,
			# So to get the "super", you need to get the Child's parent's version of `@text`.
			# By using the `::` operator, we don't leave `@text` unbound.
			$super = _1.$__parent__.$__parent__::$@text;

			# Since the text method was unbound, it work shere
			'Baby ' + super(_1)
		};

		# Children want food.
		$says_what = 'Waa! I want food!';

		__this__
	}(__this__);

	__this__
}();

$sam = Person('Sam', 'W');
$child = Child('Jace', 'B');
sam.$speak(); # => "Sam W: hi!"
child.$speak(); # => "Baby Jace B: Waa! I want food!"






