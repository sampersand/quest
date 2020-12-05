# Helper functions to support destructuring.
Text.'*@' = self -> { self.__splat__ = true; self };
Text.':' = (self, value) -> { self.__default__ = value; self };

# List destructuring. We do this by overloading `=`.
List.'=' = (self, args, scope) -> {
	# `:n` means "n strackframes up"
	scope = scope.or(:1);
	rhs = [];

	args.each({
		# `.?` means "if the rhs exists on the lhs, return `rhs.lhs`. otherwise,
		# return `null`."
		if (_0.?__splat__, {
			rhs += _0 # add the splat in
		}, {
			rhs.push(_0) # push the var onto the end.
		})
	});

	self.each({
		_0.'='(
			if (_0.?__splat__, {
				# if it's a splat operator, slurp up the rest of the arguments
				x = rhs.clone(); rhs.clear(); x
			}, {
				# if RHS isn't empty, remove the first value from `rhs`. else,
				# get the default value associated with the variable, or null.
				if (rhs, rhs.shift, { _0.?__default__ })
			}),
			scope
		)
	});
};

# Overload `->` so it'll destructure for us.
List.'->' = {
	(self, block, *'') = __args__;
	{
		self.'='(__args__, :0);
		:0.instance_exec(block)
	}
};

# Also works with functions
foo = (a, b: 3, *c, d, e: 99) -> {
	print(a, " ", b, " ", c, " ", d, " ", e);
};

foo(1);          # => 1 3 [] null 99
foo(1, 2, 3, 4); # => 1 2 [3, 4] null 99
foo(*"abcd", d: 5);   # => a b ['c', 'd'] 5 99


b = "c";


['a', 'b'].'='([1, 2]);
print(a, b)
# [a, (b)] = [1,2];
# print(c);













