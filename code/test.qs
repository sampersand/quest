# A class is just an executed block of code
Kernel.$Person = {
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
		$__parent__ = (_1.$instance_methods);

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
			(_1.$first) + ' ' + (_1.$last)
		};

		$says_what = 'hi';

		$speak = {
			disp('' + _1 + ':', _1.$says_what);
		};

		# Return this as the last argument, so we can use it as a parent.
		__this__
	}(__this__); # pass __this__ as the first argument

	__this__
}();

$Child = {
	$__parent__ = Person;

	$instance_methods = {
		$class = _1;

		$__parent__ = (_1.$instance_methods);

		$@text = {
			$super = (_1.$__parent__::$@text);
			"Baby " + super(_1)
		};

		$says_what = "Waa! I want food!";

		__this__
	}(__this__);
	__this__
}();

$sam = Person('Sam', 'W');
$child = Child('Jace', 'B');
sam.$speak();
child.$speak();




