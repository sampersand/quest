Types = {
	operator = (self, op) -> {
		__args__.set(1, 1, []);
		self::OPERATORS::(op).apply(__args__)
	};

	convert = value -> {
		if(value.__parents__.map({ _0.__id__ }).index(Number.__id__) != null, {
			return(Types::Integer(value.floor()), :1);
		});

		if(value.__parents__.map({ _0.__id__ }).index(Text.__id__) != null, {
			return(Types::String(value), :1);
		});

		if(value.__parents__.map({ _0.__id__ }).index(Boolean.__id__) != null, {
			return(Types::Boolean(value), :1);
		});

		if(null == value, {
			return(Types.Null(), :1);
		});

		assert(false, "unknown type '" + value.inspect() + "'");
	};

	:0
}();

Types.Function = {
	__parents__ = [Types];

	'()' = (class, name, args, body) -> { __parents__ = [class]; :0 };

	@text = inspect = self -> { 'func(' + self.name.@text() + ')' };

	FRAME = 0;

	call = (self, args, env) -> {
		new_env = Environment(env.globals);
		0.upto(self.args.len()-1).each({
			new_env.locals.(self.args.get(_0)) = args.get(_0);
		});

		if((Types.Function.FRAME += 1) % 40, {
			self.body.exec(new_env)
		}, {
			disp("Spawning", Types.Function.FRAME);
			spawn(self.body.exec << new_env).join()
		})
	};

	:0
}();

Types.Struct = {
	__parents__ = [Types];

	'()' = (class, name, fields) -> { __parents__ = [class]; :0 };

	@text = inspect = self -> { 'struct(' + self.name.@text() + ')' };

	Instance = {
		__parents__ = [Types];

		@text = inspect = self -> { 'instance(' + self.type.@text() + ')' };

		OPERATORS = {
			'.' = (self, arg) -> { self.(arg) };
			:0
		}();
		:0
	}();

	call = (self, args) -> {
		data = { __parents__ = [Types.Struct.Instance]; :0 }();
		data.type = self;
		0.upto(self.fields.len()).each({
			data.(self.fields.get(_0)) = args.get(_0)
		});
		data
	};

	:0
}();

Types.Integer = {
	__parents__ = [Types];

	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> { 'int(' + self.value.@text() + ')' };
	@text = self -> { self.value.@text() };

	OPERATORS = {
		'-@' = self -> { Types::convert(-self.value) };
		'~@' = self -> { Types::convert(~self.value) };
		'+@' = self -> { Types::convert(+self.value) };
		'+ - * / % ** & | ^ << >> < > <= >= == !='.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Types::convert(self.value.(op)(rhs.value))
			};
		});
		:0
	}();
	:0
}();

Types.Boolean = {
	__parents__ = [Types];

	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> { 'bool(' + self.value.@text() + ')' };
	@text = self -> { self.value.@text() };

	OPERATORS = {
		'!@' = self -> { Types::convert(!self.value) };
		'& | ^ < > <= >= == !='.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Types::convert(self.value.(op)(rhs.value))
			};
		});
		:0
	}();

	:0
}();

BoundFunction.call = BoundFunction::'()';

Types.String = {
	__parents__ = [Types];

	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> { 'string(' + self.value.@text() + ')' };
	@text = self -> { self.value.inspect() };

	shift = self -> {
		Types::convert(self.value.shift())
	};

	get = (self, where) -> {
		Types::convert(self.value.get(where.get(0).value))
	};

	OPERATORS = {
		'+ * < <= > >= == != push'.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Types::convert(self.value.(op)(rhs.value))
			};
		});
		:0
	}();

	:0
}();

Types.Null = {
	__parents__ = [Types];

	'()' = (class) -> { __parents__ = [class]; value = null; :0 };

	inspect = self -> { 'null()' };
	@text = self -> { 'null' };

	OPERATORS = {
		'!@' = self -> { Types::convert(!self.value) };
		'== !='.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Types::convert(self.value.(op)(rhs.value))
			};
		});
		:0
	}();

	:0
}();
