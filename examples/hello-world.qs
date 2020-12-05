# Simply printing it
print("Hello, world!");

# Using variables
where = "world";
print("Hello, " + where + "!");

# Using a function
greet = where -> {
	print("Hello, " + where + "!");
};

greet("world");
