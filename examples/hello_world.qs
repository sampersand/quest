# Simply printing it
disp("Hello, world!");

# Using variables
$where = "world";
disp("Hello, " + where + "!");

# Using a function
$greet = {
	disp("Hello, " + _1 + "!");
};

greet("world");