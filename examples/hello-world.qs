# Simply printing it
disp("Hello, world!");

# Using variables
where = "world";
disp("Hello, " + where + "!");

# Using a function
greet = where -> {
	disp("Hello, " + where + "!");
};

greet("world");
