# Helper functions to support destructuring.
Text.'*@' = self -> { self.__splat__ = true; self };
Text.':' = (self, value) -> { self.__value__ = value; self };

# List destructuring. We do this by overloading `=`.
List.'=' = (self, rhs, scope) -> {
	scope = scope.or(:1);
	rhs = rhs.@list().clone();
	self = self.clone();
	splat = false;

	until (self.empty?) {{
		argument = self.shift();

		argument.__has_attr__('__splat__').then {
			assert(!splat, "splat given twice!");
			:3.splat = true;

			argument.then { # ie an argument was given
				argument.'='(rhs, scope);
			};

			return(null, :2);
		};

		rhs.else {
			argument.'='(argument.?__value__, scope).return(:2);
		};

		value = rhs.shift();

		if (value.__has_attr__('__value__'), {
			self.unshift(argument);
			idx = self.index(value);
			assert(idx != null, "unknown kwarg '" + value + "' given!");
			self.delete(idx);
			value.'='(value.__value__, scope);
		}, {
			assert(!splat, "positional args given after splat!");
			argument.'='(value, scope);
		});
	}()};
};

# Overload `->` so it'll destructure for us.
List.'->' = {
	self = __args__[0];
	block = __args__[1];
	{
		self.'='(__args__, :0);
		:0.instance_exec(block)
	}
};

# Also works with functions
foo = (a, b: 3, *'', d, e: 9) -> {
	print(a, " ", b, " ", 'c', " ", d, " ", e);
};

foo(1, 2, e: 3)
