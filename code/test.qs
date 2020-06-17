# If a maximum value was passed in on the command line (_1 is program name), then use that.
# otherwise, calculate one.
$MAX = if(__has_attr__($_2), { _2 }, { prompt("max=") })().$@num();

# Calculate a secret value
$secret = rand(1, MAX).$round();
$guesses = 0;
$guess = 0;

while({ guess != secret }, {
	# we have no need for a new stackframe, so simply set ourselves to the second stackframe--the
	# outermost one. That way, it's as if we never opened a new stackframe at all.
	$__this__ = __stack__.$get(2);
	$guesses = guesses + 1;
	$guess = prompt("Pick a number from 1-" + MAX + ": ").$@num();

	# In the future, I'll implement a switch statement lol
	disp(if(secret < guess, {
		"too high!"
	}, {
		if(secret > guess, "too low!", "perfect!")
	})());
});

disp("it took you", guesses, "guesses");
disp(12.$__id__);

##__EOF__##
$MAX = prompt("max number?");
$secret = 
$i = 0;
{
	$i = 2;
	(__stack__.$get(-2)).$i = 4;
	disp(i);
}();

disp(i);









##__EOF__##
$IndexBy = {
	$__attr_missing__ = { _1.$get(_2) };
	__this__
}();

List.$__parents__.$push(IndexBy);
Text.$__parents__.$push(IndexBy);

disp (["hi, there", "friend "], 3,	 4);
disp(['hi', 'there', 'friend'].2); # => 'there'
disp("hello".2); # => 'e'

##__EOF__##
$Person = {
	$name = "Person";

	$() = {
		$__parents__ = [_1.$instance_methods];

		$first = _2;
		$last = _3;

		__this__
	};

	$instance_methods = {
		$__parents__ = [Pristine];
		$class = _1;

		$SAYS_WHAT = 'hi';

		$@text = { _1.$first + ' ' + _1.$last };
		$speak = { disp(_1, ':', _1.$SAYS_WHAT); };

		__this__
	}(__this__);

	__this__
}();

$Child = {
	$name = 'Child';
	$__parents__ = [Person]; # we're now a subclass of `Person`

	$instance_methods = {
		# We also inherit instance methods from `Person` too.
		$__parents__ = [_1.$instance_methods];
		$class = _1;

		$p = _1.$instance_methods;
		$super = { (p::_2)(_1) };

		$SAYS_WHAT = "Waa! I want food!";

		$@text = { "Baby " + _1.$super($@text) };

		__this__
	}(__this__);

	__this__
}();


$sam = Person('Sam', 'W');
$child = Child('Jace', 'B');
sam.$speak();
child.$speak();

##__EOF__##

#disp(sam.$instance_methods.$class);
#disp(sam.$FOO_BAR)

##__EOF_##



