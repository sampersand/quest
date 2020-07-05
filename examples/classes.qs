# This is how you would represent classical "classes" within Quest by having
# the "class" be its own object too.

$Person = {
	# The `@text` attribute for scopes uses the `name` attribute if it exists.
	$name = "Person";

	$() = {
		$__parents__ = [Person.$instance_methods];

		$first = _1;
		$last = _2;

		:0 # return the current stackframe
	};

	$instance_methods = {
		# set parent to the calling stackframe
		$class = :1;

		$@text = {
			_0.$first + " " + _0.$last
		};

		:0 # return the current stackframe
	}();

	:0 # return the current stackframe
}();

$sam = Person("Sam", "W");

disp(Person); # => Person
disp(sam); # => Sam W
disp(sam.$class == Person); # => true
