Io::File('tokenizer.qs').read().eval();
Io::File('types.qs').read().eval();

Environment = {
	DEFAULT_GLOBALS = {
		__parents__ =  [Pristine];
		fread = (args, env) -> {
			filename = args.get(0).value;
			read_what = args.get(1).then({ _0.value }).else({ "\n" });
			Io.File(filename, 'r').read(read_what)
		};

		fwrite = (args, env) -> {
			Io.File(args.get(0).value, 'w').write(args.get(1).value)
		};

		print = (args, env) -> { disp(args.get(0)); Types::convert(null) };
		print.__parents__.get(0).call = print::'()';
		:0
	}();

	'()' = (class, globals) -> {
		__parents__ = [class];
		globals = if(null == globals, { Environment::DEFAULT_GLOBALS }, { globals });
		locals = { __parents__ = [Pristine]; :0 }();
		:0
	};
	:0
}();

take_while = fn -> {
	list = [];
	{loop({
		x = fn();
		if(x == null, { return(null, :2) });
		list.push(x);
	})}();
	list
};

Expression = { :0 }();
Expression.Assignment = {
	'()' = (class, ident, op, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'Assignment(' + self.ident + ', ' + self.op + ', ' + self.value.inspect() + ')'
	};

	@text = self -> { self.ident + ' ' + self.op + ' ' + self.value.@text() };

	parse = (class, parser) -> {
		ident = parser.next_if_type('IDENTIFIER').else(return);

		op = parser.next_if_type('OPERATOR_ASSIGN').else({
			parser.put_back(ident);
			return(null, :1);
		});

		value = parser.expr().assert('missing RHS of assignment');

		class(ident.value, op.value, value)
	};

	exec = (self, env) -> {
		rhs = self.value.exec(env);

		if(self.op != '=', {
			lhs = env.locals.(self.ident);
			:1.rhs = env.locals.(self.ident).operator(self.op.get(0,-2), rhs);
		});
		env.locals.(self.ident) = rhs
	};
	:0
}();

Expression.If = {
	'()' = (class, cond, if_true, if_false) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'If(' + self.cond.inspect()
			+ ', ' + self.if_true.inspect()
			+ ', ' + self.if_false.inspect() + ')'
	};

	@text = self -> {
		base = 'if ' + self.cond.@text() + self.if_true.@text();
		self.if_false.then({
			base += ' else ' + self.if_false.@text();
		});
		base
	};

	parse = (class, parser) -> {
		parser.next_if_token(Token::IF).else(return);
		cond = parser.expr().assert('missing condition for IF');
		if_true = Expression::Block.parse(parser).assert('missing if_true block');
		if_false = null;

		parser.next_if_token(Token::ELSE).then({
			:1.if_false = Expression::Block.parse(parser).assert('missing if_false block');
		});

		class(cond, if_true, if_false)
	};

	exec = (self, env) -> {
		if(self.cond.exec(env).value, self.if_true.exec << env, {
			self.if_false.then({ self.if_false.exec(env) })
		})
	};

	:0
}();

Expression.While = {
	'()' = (class, cond, body) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'While(' + self.cond.inspect() + ', ' + self.body.inspect() + ')'
	};

	@text = self -> {
		'while ' + self.cond.@text() + self.body.@text()
	};

	parse = (class, parser) -> {
		parser.next_if_token(Token::WHILE).else(return);

		cond = parser.expr().assert('missing condition for WHILE');
		body = Expression::Block.parse(parser).assert('missing WHILE block');

		class(cond, body)
	};

	exec = (self, env) -> {
		while({ self.cond.exec(env).value }, {
			self.body.exec(env)
		})
	};

	:0
}();

Expression.Return = {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'Return(' + self.value.inspect() + ')'
	};

	@text = self -> {
		if(self.value, {
			'return ' + self.value.@text()
		}, {
			'return'
		})
	};

	parse = (class, parser) -> {
		parser.next_if_token(Token::RETURN).then({ class(parser.expr()) })
	};

	exec = (self, env) -> { quit(1, 'todo: return'); };

	:0
}();

Expression.Continue = {
	'()' = (class) -> { __parents__ = [class]; :0 };

	inspect = self -> { 'Continue()' };
	@text = self -> { 'continue' };

	parse = (class, parser) -> {
		parser.next_if_token(Token::CONTINUE).then(class)
	};

	exec = (self, env) -> { quit(1, 'todo: continue'); };

	:0
}();

Expression.Break = {
	'()' = (class) -> { __parents__ = [class]; :0 };

	inspect = self -> { 'Break()' };
	@text = self -> { 'break' };

	parse = (class, parser) -> {
		parser.next_if_token(Token::BREAK).then(class)
	};

	exec = (self, env) -> { quit(1, 'todo: break'); };

	:0
}();

Expression.FuncDecl = {
	'()' = (class, name, args, body) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'FuncDecl(' + self.name + ', ' + self.args.inspect() + ', ' + self.body.inspect() + ')'
	};

	@text = self -> {
		'func ' + self.name + '(' + self.args.join(', ') + ') ' + self.body.@text()
	};

	parse = (class, parser) -> {
		parser.next_if_token(Token::FUNC).else(return);

		name = parser.next_if_type('IDENTIFIER').and_then({ _0.value });
		parser.next_if_token(Token::LPAREN).assert('missing start of function arguments');

		args = take_while({
			parser.next_if_token(Token::RPAREN).then(return);

			arg = parser.next_if_type('IDENTIFIER').assert('invalid argument name').value;
			parser.next_if_token(Token::COMMA);
			arg
		});

		body = Expression::Block.parse(parser).assert('missing function body');
		class(name, args, body)
	};

	exec = (self, env) -> {
		type = Types::Function(self.name, self.args, self.body);

		if(self.name, { env.globals.(self.name) = type });

		type
	};

	:0
}();

Expression.StructDecl = {
	'()' = (class, name, fields) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'StructDecl(' + self.name + ', ' + self.fields.inspect() + ')'
	};

	@text = self -> { 'struct ' + self.name + ' { ' + self.fields.join(', ') + ' }' };

	parse = (class, parser) -> {
		parser.next_if_token(Token::STRUCT).else(return);

		name = parser.next_if_type('IDENTIFIER').assert('missing struct name').value;
		parser.next_if_token(Token::LBRACE).assert('missing start of struct arguments');

		fields = take_while({
			parser.next_if_token(Token::RBRACE).then(return);

			arg = parser.next_if_type('IDENTIFIER').assert('invalid field name').value;
			parser.next_if_token(Token::COMMA);
			arg
		});

		class(name, fields)
	};

	exec = (self, env) -> {
		env.globals.(self.name) = Types::Struct(self.name, self.fields)
	};

	:0
}();

Expression.FuncCall = {
	'()' = (class, func, args) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'FuncCall(' + self.func.inspect() + ', ' + self.args.inspect() + ')'
	};

	@text = self -> {
		self.func.@text() + '(' + self.args.join(', ') + ')'
	};

	parse = (class, parser, func) -> {
		parser.next_if_token(Token::LPAREN).else(return);

		args = take_while({
			parser.next_if_token(Token::RPAREN).then(return);
			arg = parser.expr().assert('missing expression in func call');
			parser.next_if_token(Token::COMMA);
			arg
		});

		class(func, args)
	};

	exec = (self, env) -> {
		func = self.func.exec(env);
		args = self.args.map({ _0.exec(env) });

		func.call(args, env)
	};

	:0
}();

Expression.Integer = {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'Integer(' + self.value.inspect() + ')'
	};

	@text = self -> { self.value.@text() };

	parse = (class, parser) -> {
		token = parser.next_if_type('INTEGER').else(return);

		class(token.value.@num())
	};

	exec = (self, _) -> { Types::Integer(self.value) };

	:0
}();

Expression.Identifier = {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'Identifier(' + self.value.inspect() + ')'
	};

	@text = self -> { self.value.@text() };

	parse = (class, parser) -> {
		token = parser.next_if_type('IDENTIFIER').else(return);

		class(token.value)
	};

	exec = (self, env) -> {
		if(env.locals.__has_attr__(self.value), {
			env.locals::(self.value)
		}, {
			env.globals::(self.value)
		})
	};

	:0
}();

Expression.Boolean = {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'Boolean(' + self.value.inspect() + ')'
	};

	@text = self -> { self.value.@text() };

	parse = (class, parser) -> {
		token = parser.next_if_type('BOOLEAN').else(return);

		assert((token.value == 'true').else(token.value.'==' << 'false'));

		class(token.value == 'true')
	};

	exec = (self, _) -> { Types::Boolean(self.value) };

	:0
}();

Expression.String = {
	'()' = (class, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'String(' + self.value.inspect() + ')'
	};

	@text = self -> { self.value.inspect() };

	parse = (class, parser) -> {
		token = parser.next_if_type('STRING').else(return);

		class(token.value)
	};

	exec = (self, _) -> { Types::String(self.value) };
	:0
}();

Expression.Null = {
	'()' = class -> { __parents__ = [class]; value = null; :0 };

	inspect = self -> { 'Null()' };

	@text = self -> { self.value.@text() };

	parse = (class, parser) -> {
		parser.next_if_token(Token::NULL).then(class)
	};

	exec = (self, _) -> { Types::Null() };

	:0
}();

Expression.UnaryOp = {
	'()' = (class, op, arg) -> { __parents__ = [class]; op += '@'; :0 };

	inspect = self -> {
		'UnaryOp(' + self.op + ', ' + self.arg.inspect() + ')'
	};

	@text = self -> { self.op.@text() + '(' + self.arg.@text() + ')' };

	parse = (class, parser) -> {
		op = parser.next_if({ 
			[Token::ADD, Token::SUB, Token::NOT, Token::BNOT].index(_0) != null
		}).else(return);

		arg = parser.expr().assert('missing RHS to operator');
		class(op.value, arg)
	};

	exec = (self, env) -> {
		self.arg.exec(env).operator(self.op)
	};

	:0
}();

Expression.StructIndexAssign = {
	'()' = (class, struct, field, oper, value) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'StructIndexAssign(' + self.struct.inspect()
			+ ', ' + self.field
			+ ', ' + self.oper
			+ ', ' + self.value.inspect() + ')'
	};

	@text = self -> { self.struct.@text() + '.' + self.field + ' = ' + self.vale.@text() };

	exec = (self, env) -> {
		(self.struct.exec(env)).(self.field) = self.value.exec(env)
	};

	:0
}();

Expression.StructIndex = {
	'()' = (class, struct, field) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'StructIndex(' + self.struct.inspect() + ', ' + self.field + ')'
	};

	@text = self -> { self.struct.@text() + '.' + self.field };

	parse = (class, parser, lhs) -> {
		parser.next_if_token(Token::DOT).else(return);
		field = parser.next_if_type('IDENTIFIER').assert('missing fieldname for struct index').value;
		oper = self.next_if_type('OPERATOR_ASSIGN');

		oper.then({
			value = parser.expr().assert('no rhs for struct index assign');

			return(Expression.StructIndexAssign(lhs, field, oper.value, value), :2);
		});

		class(lhs, field)
	};

	exec = (self, env) -> {
		(self.struct.exec(env)).(self.field)
	};

	:0
}();

Expression.BinaryOp = {
	'()' = (class, op, lhs, rhs) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'BinaryOp(' + self.op + ', ' + self.lhs.inspect() + ', ' + self.rhs.inspect() + ')'
	};

	@text = self -> {
		'(' + self.lhs.@text() + ') ' + self.op + ' (' + self.rhs.@text() + ')'
	};

	parse = (class, parser, lhs) -> {
		op = parser.next_if_type('OPERATOR').else(return);
		rhs = parser.expr().assert('missing RHS for op ' + op);

		class(op.value, lhs, rhs)
	};

	exec = (self, env) -> {
		self.lhs.exec(env).operator(self.op, self.rhs.exec(env))
	};

	:0
}();


Expression.ShortCircuitOp = {
	'()' = (class, op, lhs, rhs) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'ShortCircuitOp(' + self.op + ', ' + self.lhs.inspect() + ', ' + self.rhs.inspect() + ')'
	};

	@text = self -> {
		'(' + self.lhs.@text() + ') ' + self.op + ' (' + self.rhs.@text() + ')'
	};

	parse = (class, parser, lhs) -> {
		op = parser.next_if_type('SHORT_CIRCUIT').else(return).value;
		rhs = parser.expr().assert('missing RHS for op ' + op);

		class(op, lhs, rhs)
	};

	exec = (self, env) -> {
		lhs = self.lhs.exec(env);
		if(self.op == '&&', {
			if(lhs.value, lhs.itself, self.rhs.exec << env)
		}, {
			if(lhs.value, self.rhs.exec << env, lhs.itself)
		})
	};

	:0
}();

Expression.Block = {
	'()' = (class, body) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'Block(' + self.body.inspect() + ')';
	};

	@text = self -> {
		'{' + self.body.map({"\n\t" + _0 }).join() + "\n}"
	};

	parse = (class, parser) -> {
		parser.next_if_token(Token::LBRACE).else(return);

		args = take_while({
			parser.next_if_token(Token::RBRACE).then(return);
			parser.expr().assert('missing expression in function call')
		});

		class(args)
	};

	exec = (self, env) -> {
		self.body.map({ _0.exec(env) }).get(-1)
	};

	:0
}();

Parser = {
	'()' = (class, source) -> {
		__parents__ = [class];
		peeked = [];
		tokenizer = Tokenizer(source);
		:0.__del_attr__('source');
		:0
	};

	next_if = (self, fn) -> { self.peek_token().and_then(fn).then(self.next_token) };
	next_if_token = (self, token) -> { self.next_if(token.'==') };
	next_if_type = (self, type) -> { self.next_if({ _0.type == type }) };

	put_back = (self, value) -> {
		self.peeked.push(value);
	};

	next_token = self -> {
		self.peeked.pop().else(self.tokenizer.next)
	};

	peek_token = self -> {
		unless(self.peeked, {
			self.put_back(self.next_token());
		});

		self.peeked.get(-1)
	};

	inspect = self -> { 'Parser(' + self.peeked.inspect() + ', ' + self.tokenizer.inspect() + ')' };

	@bool = self -> {
		self.peeked.else(self.tokenizer.itself).@bool()
	};

	@list = self -> {
		take_while(self.expr)
	};

	primary = self -> {
		Expression::Assignment.parse(self)
			.else(self.kw_expr)
			.else(self.literal)
			.else(self.paren_expr)
			.else(self.block)
	};

	expr = self -> {
		self.else(return);
		lhs = self.primary().else(return);

		self.next_if_token(Token::SEMICOLON).then(lhs.return);

		self.expr_rhs(lhs)
	};

	expr_rhs = (self, lhs) -> {
		expr = Expression::BinaryOp.parse(self, lhs)
			.else(Expression::ShortCircuitOp.parse << self << lhs)
			.else(Expression::FuncCall.parse << self << lhs)
			.else(Expression::StructIndex.parse << self << lhs)
			.else(lhs.return);
		self.expr_rhs(expr)
	};

	literal = self -> {
		Expression::Integer.parse(self)
			.else(Expression::Boolean.parse << self)
			.else(Expression::String.parse << self)
			.else(Expression::Null.parse << self)
			.else(Expression::Identifier.parse << self)
			.else(Expression::UnaryOp.parse << self)
	};

	kw_expr = self -> {

		Expression::If.parse(self)
			.else(Expression::While.parse << self)
			.else(Expression::Return.parse << self)
			.else(Expression::Continue.parse << self)
			.else(Expression::Break.parse << self)
			.else(Expression::FuncDecl.parse << self)
			.else(Expression::StructDecl.parse << self)
	};

	paren_expr = self -> {
		self.next_if_token(Token::LPAREN).else(return);
		ret = self.expr().assert('missing expression within paren_expr');
		self.next_if_token(Token::RPAREN).assert('Missing closing rparen for paren_expr');
		ret
	};

	block = self -> {
		self.next_if_token(Token::LBRACE).else(return);
		take_while({
			ret = self.expr().assert('missing expression within paren_expr');
			self.next_if_token(Token::RBRACE).assert('Missing closing rparen for paren_expr');
			ret
		})
	};

	run = (self, env) -> {
		self.@list().each({ _0.exec(env) })
	};

	:0
}();
