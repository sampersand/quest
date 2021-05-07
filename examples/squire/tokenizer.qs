Basic.and_then = (self, func) -> {
	self.then(func.'()' << self)
};

Text.shift_re = (self, regex) -> {
	match = regex.match(self).else(return).get(0);

	unless(self.get(0, match.len() - 1) == match, return);

	self.replace(self.get(match.len(), -1).or(''));

	match
};

Token = class() {
	'()' = (class, type, value) -> { __parents__ = [class]; :0 };

	@text = self -> {
		self.value.@text()
	};

	inspect = self -> {
		self.type + '(' + self.value.inspect() + ')'
	};

	'==' = (self, rhs) ->  {
		(self.type == rhs.type).then({ self.value == rhs.value })
	};

	RETURN = :0('KEYWORD', 'return');
	IF = :0('KEYWORD', 'if'); ELSE = :0('KEYWORD', 'else');
	WHILE = :0('KEYWORD', 'while'); FOR = :0('KEYWORD', 'for');
	CONTINUE = :0('KEYWORD', 'continue'); BREAK = :0('KEYWORD', 'break');
	FUNC = :0('KEYWORD', 'func'); STRUCT = :0('KEYWORD', 'struct');

	TRUE = :0('BOOLEAN', 'true'); FALSE = :0('BOOLEAN', 'false'); NULL = :0('NULL', 'null');

	LPAREN = :0('PAREN', '(');
	RPAREN = :0('PAREN', ')');
	LBRACE = :0('PAREN', '{');
	RBRACE = :0('PAREN', '}');
	LBRACKET = :0('PAREN', '[');
	RBRACKET = :0('PAREN', ']');

	COMMA = :0('COMMA', ',');
	SEMICOLON = :0('SEMICOLON', ';');
	EQUALSIGN = :0('OPERATOR_ASSIGN', '=');

	ADD = :0('OPERATOR', '+'); ADD_EQ = :0('OPERATOR_ASSIGN', '+=');
	SUB = :0('OPERATOR', '-'); SUB_EQ = :0('OPERATOR_ASSIGN', '-=');
	MUL = :0('OPERATOR', '*'); MUL_EQ = :0('OPERATOR_ASSIGN', '*=');
	DIV = :0('OPERATOR', '/'); DIV_EQ = :0('OPERATOR_ASSIGN', '/=');
	MOD = :0('OPERATOR', '%'); MOD_EQ = :0('OPERATOR_ASSIGN', '%=');
	POW = :0('OPERATOR', '**'); POW_EQ = :0('OPERATOR_ASSIGN', '**=');

	BNOT = :0('OPERATOR_UNARY', '~');
	BAND = :0('OPERATOR', '&'); BAND_EQ = :0('OPERATOR_ASSIGN', '&=');
	BOR  = :0('OPERATOR', '|');  BOR_EQ  = :0('OPERATOR_ASSIGN', '|=');
	BXOR = :0('OPERATOR', '^'); BXOR_EQ = :0('OPERATOR_ASSIGN', '^=');
	BSHL = :0('OPERATOR', '<<'); BSHL_EQ = :0('OPERATOR_ASSIGN', '<<=');
	BSHR = :0('OPERATOR', '>>'); BSHR_EQ = :0('OPERATOR_ASSIGN', '>>=');

	NOT = :0('OPERATOR_UNARY', '!');
	LTH = :0('OPERATOR', '<'); LEQ = :0('OPERATOR', '<=');
	GTH = :0('OPERATOR', '>'); GEQ = :0('OPERATOR', '>=');
	EQL = :0('OPERATOR', '=='); NEQ = :0('OPERATOR', '!=');

	AND = :0('SHORT_CIRCUIT', '&&');
	OR = :0('SHORT_CIRCUIT', '||');
	DOT = :0('MISC', '.');

	# Look Ma, currying!
	Identifier = :0.'()' << 'IDENTIFIER';
	Integer    = :0.'()' << 'INTEGER';
	String     = :0.'()' << 'STRING';

	Paren = {
		[LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET].each({
			:1.(_0.value) = _0
		});
		:0.'.'
	}();

	Keyword = {
		[RETURN, IF, ELSE, WHILE, FOR, CONTINUE, BREAK, FUNC, STRUCT, TRUE, FALSE, NULL].each({
			:1.(_0.value) = _0
		});
		:0.'.'
	}();

	Operator = {
		dot = :0.'.';

		[ADD, ADD_EQ, SUB, SUB_EQ, MUL, MUL_EQ, DIV, DIV_EQ, MOD, MOD_EQ, POW, POW_EQ,
		 BNOT, BAND, BAND_EQ, BOR, BOR_EQ, BXOR BXOR_EQ BSHL BSHL_EQ BSHR BSHR_EQ,
		 NOT, LTH, LEQ, GTH, GEQ, EQL, NEQ, EQUALSIGN].each({
		 :1.(_0.value) = _0;
	 	});

		dot
	}();
};

Tokenizer = class() {
	'()' = (class, source) -> { __parents__ = [class]; :0 };

	inspect = self -> {
		'Tokenizer(' + self.source.inspect() + ')'
	};

	@bool = self -> {
		self.source.@bool()
	};

	next = self -> {
		self.strip_comments_and_whitespace();

		/\A\s*__END__\n/m.match(self.source).then(return);
		self.else(return);

		self.misc()
			.else(self.bool_or_null)
			.else(self.keyword)
			.else(self.ident)
			.else(self.operator)
			.else(self.integer)
			.else(self.string)
	};

	@list = self -> {
		x=[];

		while ({ :1.v = self.next() }) {
			x.push(v)
		};

		x
	};

	strip_comments_and_whitespace = self -> {
		while (self.source.shift_re << /\A(?:\s+|#.*\n)/) {
			# do nothing
		};
	};

	keyword = self -> {
		self.source
			.shift_re(/\A(if|else|return|while|for|continue|break|func|struct)\b/)
			.and_then(Token::Keyword)
	};

	ident = self -> {
		self.source
			.shift_re(/\A[a-zA-Z_]\w*\b/)
			.and_then(Token::Identifier)
	};

	integer = self -> {
		self.source
			.shift_re(/\A\d+\b/)
			.and_then(Token::Integer)
	};

	bool_or_null = self -> {
		self.source.shift_re(/\Atrue\b/).then(Token::TRUE.return);
		self.source.shift_re(/\Afalse\b/).then(Token::FALSE.return);
		self.source.shift_re(/\Anull\b/).then(Token::NULL.return);
	};

	string = self -> {
		ret = self.source
			.shift_re(/\A"(?:\\\\x[a-fA-F0-9]{2}|\\[nrtf'"\\\\0]|[^"])*"/)
			.else(self.source.shift_re << /\A'(?:\\\\x[a-fA-F0-9]{2}|\\[nrtf'"\\\\0]|[^'])*'/)
			.else(return);
		x = ret.eval(); # eval's a hacky way to interpolate escapes correctly.
		Token::String(ret.eval()) # eval's a hacky way to interpolate escapes correctly.
	};

	operator = self -> {
		self.source
			.shift_re(/\A
				(?:[-+*\/%]|\*\*)=?|  # mathematic operators
				(?:[&|^]|>>|<<)=?|    # bitwise operators
				(?:[=!<>]=?)|         # comparison operators and assignment
				[!~] 						 # remaining unary operators
			/x)
			.and_then(Token::Operator)
	};

	misc = self -> {
		self.source.shift_re(/\A[(){}\[\]]/).and_then({ return(Token::Paren(_0), :2) });
		self.source.shift_re(/\A,/).then(Token::COMMA.return << :1);
		self.source.shift_re(/\A;/).then(Token::SEMICOLON.return << :1);
		self.source.shift_re(/\A\./).then(Token::DOT.return << :1);
		self.source.shift_re(/\A\|\|/).then(Token::AND.return << :1);
		self.source.shift_re(/\A\&\&/).then(Token::OR.return << :1);
	};
};
