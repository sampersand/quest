$first = 'Sam'; $last = 'Westerman';

# Create a person class. Classes are created by executing a block that returns
# `__this__`. (Note: $FOO is identical to "FOO"---it's just syntactic sugar)
$Person = {

	# Define what it means to "call" a Person (ie `Person(...)`)
	"()" = {

		# Set the current object's parent to `Person` so we can have access to
		# its `@text` method.
		$__parent__ = Person;
		
		# Assign first name to the first argument and last name to the second.
		$first = _1;
		$last = _2;

		__this__ # Return the current object, which we just created in this method.
	};

	# Define the conversion function to text
	"@text" = {
		first + ' ' + last
	};

	# As this is a class, we return `__this__` at the end...
	__this__
}(); # ... and immediately execute the block so as to create the class.

# Assign me as a new human.
$sam = Person('sam', 'westerman');
# And greet me...
disp("Hello, " + sam);


# (12.3."floor")(12.3)

# disp(Person("Sam"))
# # # $_if = if; $if = {
# # # 	_if('if', (__args__.'[]')(__args__, 1), (__args__.'[]')(__args__, 2), (__args__.'[]')(__args__, 3))
# # # };

# # # $_while = while; $while = {
# # # 	_while('while', (__args__.'[]')(__args__, 1), (__args__.'[]')(__args__, 2))
# # # };

# # # Number."<" = {
# # # 	(_1 <=> _2) == -1
# # # };

# # # $count_to = {
# # # 	$max = _1;
# # # 	$n = if(_2, _2, 1);
# # # 	if(n < max, {
# # # 		disp(n);
# # # 		count_to(max, n + 1);
# # # 	})()
# # # };

# # # count_to(10, 3);

# # # $factorial = {
# # # 	$num = _1;

# # # 	if(1 < num, {
# # # 		num * factorial(num - 1)
# # # 	}, {
# # # 		1
# # # 	})()
# # # };

# # # factorial(12)

# $Person = {
# 	'name' = 'Person';
# 	'()' = {
# 		$__parent__ = Person;
# 		$first = (__args__::'[]')(__args__, 2);
# 	};

# 	'@text' = {
# 		$this = (__args__::'[]')(__args__, 1);
# 		(this.'first') + ' ' + (this.'last')
# 	};

# 	__this__
# }();


# 'p' = Person('sam', 'westerman');
# disp(p)
# # disp((p.'@textt')(p), p.'name');


