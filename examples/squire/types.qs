Type = class() {
	operator = (self, op) -> {
		__args__.set(1, 1, []);
		self::OPERATORS::(op).apply(__args__)
	};

	convert = value -> {
		if (value.__parents__.map({ _0.__id__ }).index(Number.__id__) != null) {
			return(Type::Integer(value.floor()), :1);
		};

		if (value.__parents__.map({ _0.__id__ }).index(Text.__id__) != null) {
			return(Type::String(value), :1);
		};

		if (value.__parents__.map({ _0.__id__ }).index(Boolean.__id__) != null) {
			return(Type::Boolean(value), :1);
		};

		if (null == value) {
			return(Type.Null(), :1);
		};

		assert(false, "unknown type '" + value.inspect() + "'");
	};
};

Type.Function = class(Type) {
	'()' = (class, name, args, body) -> { __parents__ = [class]; :0 };

	@text = inspect = self -> {
		'func(' + self.name.@text() + ')'
	};

	# Hack to spawn new threads lol
	FRAME = 0;

	call = (self, args, env) -> {
		new_env = Environment(env.globals);
		0.upto(self.args.len()-1).each({
			new_env.locals.(self.args.get(_0)) = args.get(_0);
		});

		if((Type.Function.FRAME += 1) % 40, {
			self.body.exec(new_env)
		}, {
			disp("Spawning", Type.Function.FRAME);
			spawn(self.body.exec << new_env).join()
		})
	};
};

Type.Struct = class(Type) {
	'()' = (class, name, fields) -> { __parents__ = [class]; :0 };

	@text = inspect = self -> {
		'struct(' + self.name.@text() + ')'
	};

	Instance = class(Type) {
		@text = inspect = self -> { 'instance(' + self.type.@text() + ')' };

		OPERATORS = class() {
			'.' = (self, arg) -> { self.(arg) };
		};
	};

	call = (self, args) -> {
		data = class(Type.Struct.Instance) {};
		data.type = self;
		0.upto(self.fields.len()).each({
			data.(self.fields.get(_0)) = args.get(_0)
		});
		data
	};
};

Type.Integer = class(Type) {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'int(' + self.value.@text() + ')'
	};

	@text = self -> {
		self.value.@text()
	};

	OPERATORS = class() {
		'-@' = self -> { Type::convert(-self.value) };
		'~@' = self -> { Type::convert(~self.value) };
		'+@' = self -> { Type::convert(+self.value) };
		'+ - * / % ** & | ^ << >> < > <= >= == !='.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Type::convert(self.value.(op)(rhs.value))
			};
		});
	};
};

Type.Boolean = class(Type) {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'bool(' + self.value.@text() + ')'
	};

	@text = self -> {
		self.value.@text()
	};

	OPERATORS = class() {
		'!@' = self -> { Type::convert(!self.value) };
		'& | ^ < > <= >= == !='.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Type::convert(self.value.(op)(rhs.value))
			};
		});
	};
};

BoundFunction.call = BoundFunction::'()';

Type.String = class(Type) {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'string(' + self.value.@text() + ')'
	};

	@text = self -> {
		self.value.inspect()
	};

	shift = self -> {
		Type::convert(self.value.shift())
	};

	get = (self, where) -> {
		Type::convert(self.value.get(where.get(0).value))
	};

	OPERATORS = class() {
		'+ * < <= > >= == != push'.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Type::convert(self.value.(op)(rhs.value))
			};
		});
	};
};

Type.Null = class(Type) {
	__parents__ = [Type];

	'()' = (class) -> { __parents__ = [class]; value = null; :0 };

	inspect = self -> {
		'null()'
	};

	@text = self -> {
		'null'
	};

	OPERATORS = class() {
		'!@' = self -> {
			Type::convert(!self.value)
		};

		'== !='.split(' ').each(op -> {
			:1.(op) = (self, rhs) -> {
				Type::convert(self.value.(op)(rhs.value))
			};
		});
	};
};
