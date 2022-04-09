# List destructuring, by defining `=` for lists.
List.'=' = (self, args, scope) -> {
	scope = scope.or(__stack__[1]);
	args = args.clone();

	self.each(var -> {
		var.'='(args.shift(), scope)
	})
};

ary = [3, [4, 5], 6, 7];
(a, (b, c, d)) = ary;
print("(1) ", a, " ",  b, " ", c, " ", d);

# The above is actually syntactical sugar for:
['a', ['b', 'c', 'd']] = ary;
print("(2) ", a, " ",  b, " ", c, " ", d);
