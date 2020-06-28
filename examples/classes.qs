# This is how you would represent classical "classes" within Quest by having
# the "class" be its own object too.

$Person = {
	# The `@text` attribute for scopes uses the `name` attribute if it exists.
	$name = "Person";

	$() = {
		$__parents__ = [Person.$instance_methods];

		$first = _1;
		$last = _2;

		__this__
	};

	$instance_methods = {
		# so you can do `my_person.$class`
		$class = _0;

		$@text = {
			_0.$first + " " + _0.$last
		};

		__this__
	}(__this__);

	__this__
}();

$sam = Person("Sam", "W");

disp(Person); # => Person
disp(sam); # => Sam W
disp(sam.$class == Person); # => true

