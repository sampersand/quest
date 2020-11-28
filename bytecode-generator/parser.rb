require_relative 'lexer'
require_relative 'ast'

class Parser
	def initialize(lexer)
		@lexer = lexer
		@peeked = []
	end

	def top
		statements
	end

	private

	def statements
		body = []
		loop do
			stmt = expression and body.push stmt
			break unless endline
		end
		Ast::Statements.new body
	end

	def endline
		token(:endline, ignore_soft: false)
	end

	# expression := terminal | assignment | function-call | expression BINARY_OP expression ;
	def expression
		term = terminal

		while (tmp = assignment(term) || function_call(term) || binary_op(term))
			term = tmp
		end

		term
	end

	# assignment := [expression '.'] ident-or-paren ASSIGNMENT_OP expression ;
	def assignment(expr)
		if expr.is_a?(Ast::Binary) && expr.op == '.'
			asgn = assignment_op or return
			rvalue = expression or error 'missing expression after assignment op'

			unless asgn.value == '='
				rvalue = Ast::Binary.new(asgn.value[..-2], expr.dup, rvalue)
			end
			# TODO: `.+=` doesn't make much sense...
			Ast::FieldAssignment.new(expr.lhs, expr.rhs, rvalue)
		elsif asgn = assignment_op
			rvalue = expression or error 'missing expression after assignment op'
			expr = Ast::Identifier.new(expr.value) if expr.is_a?(Ast::Variable)
			Ast::Assignment.new expr, rvalue
		end
	end

	def binary_op(expr)
		if dot
			rhs = identifier || expression or error 'dot without a rhs?'
			Ast::Binary.new '.', expr, rhs
		elsif tkn = token(:op)
			rhs = expression or error '+ without a rhs?'
			Ast::Binary.new tkn.value, expr, rhs
		else
			nil
		end
	end

	def function_call(expr)
		return unless lparen
		args, kwargs = function_call_args
		error 'expecting rparen after function call args' unless rparen
		Ast::FunctionCall.new(expr, args, kwargs)
	end

	def function_call_args
		args = []
		kwargs = []
		loop do
			case
			when splat 
				error 'unexpected splat' unless kwargs.empty?
				list = expression or error 'splat without expression'
				args.push Ast::FunctionCall::Splat.new list
			when splatsplat 
				map = expression or error 'splatsplat without expression'
				kwargs.push Ast::FunctionCall::SplatSplat.new map
			when (expr = expression || break) && colon
				rhs = expression || expr
				expr = Ast::Text.new expr.value if expr.is_a? Ast::Variable
				kwargs.push Ast::FunctionCall::Kwarg.new expr, rhs
			else
				error 'unexpected positional argument' unless kwargs.empty?
				args.push Ast::FunctionCall::Positional.new expr
			end
			comma || break # if we're not given a comma, that means we're done
		end
		[args, kwargs]
	end

	# terminal := literal | unary | paren-phrase | bracket-literal | lambda ;
	def terminal
		literal || unary || paren_phrase || bracket_literal || lambda
	end

	# literal := text | number | variable | regex | stackframe ;
	def literal
		text || number || variable || regex || stackframe
	end

	def unary
		operator = token(:op, '+') || token(:op, '-') || token(:op, '!') || token(:op, '~') or return
		arg = expression or raise "missing arg for unary op '#{operator.value}'"
		Ast::Unary.new(operator.value, arg)
	end

	# paren-phrase := '(' statements ')'
	def paren_phrase
		return unless lparen
		body = statements
		error 'missing closing rparen' unless rparen
		Ast::ParenPhrase.new(body)
	end

	# bracket-literal := '[' [ (list-items | map-items) [','] ] ']' ;
	def bracket_literal
		return unless lbracket
		body = list_items || map_items
		comma if body # ignore the optional trailing comma, if a body was given.
		body ||= (colon ? Ast::Map : Ast::List).new []
		error 'missing closing rbracket' unless rbracket
		body
	end

	# list-items := ['*'] expression [',' list-items] ;
	def list_items(can_be_map: true)
		items = []
		# optional splat to start with
		if splat
			list = expression or error 'expected expression after splat'
			items.push Ast::List::Splat.new list
		else
			exp = expression or return

			if can_be_map && c = colon
				put_back c
				return map_items exp
			end

			items.push Ast::List::Item.new(exp)
		end

		# if we have a comma, we might have more items
		if (cma = comma) 
			if (list = list_items)
				# if we're able to parse another list item, great! add it onto our current list
				items += list.items
			else
				# otherwise, the comma's a part of another construct, so put it back.
				put_back cma
			end
		end

		# regardless, we now return the list of elements.
		Ast::List.new items
	end

	# map-items := ( ['**'] expression | ident-or-paren ':' [expression] ) [',' map-items] ;
	def map_items(key=nil)
		# optional splatsplat to begin with
		fields = []

		if splatsplat
			map = expression or error 'expected expression after splatsplat'
			fields.push Ast::Map::SplatSplat.new map
		else
			key ||= paren_or_ident or return
			colon or error 'expected colon after key'
			if (value = expression)
				if key.is_a?(Ast::Identifier) || key.is_a?(Ast::Variable)
					key = Ast::Text.new key.value
				end
			else
				if key.is_a?(Ast::Identifier) || key.is_a?(Ast::Variable)
					key = Ast::Text.new key.value
					value = Ast::Variable.new key.value
				else
					# TODO: lookup `value` in the environment
					error 'expected expression after colon or identifier before it.'
				end
			end

			fields.push Ast::Map::Field.new key, value
		end

		# if we have a comma, we might have more items
		if (cma = comma) 
			if (map = map_items)
				# if we're able to parse another list item, great! add it onto our current list
				fields += map.fields
			else
				# otherwise, the comma's a part of another construct, so put it back.
				put_back cma
			end
		end

		Ast::Map.new fields
	end

	def paren_or_variable
		paren_phrase || variable
	end

	def paren_or_ident
		paren_phrase || identifier
	end

	# lambda := '{' ['|' lambda-args '|'] statements '}' ;
	def lambda
		lbrace or return

		if lambda_args_start
			args = lambda_args # we're allowed to have nil args.
			lambda_args_end or error 'expected lambda_args_end after optional lambda_args'
		end

		body = statements or fail '<internal error: statements always returns non-nil>'
		rbrace or error 'expected rbrace to end lambda'
		Ast::Lambda.new args, body
	end

	# lambda-args := [lambda-required] ;
	def lambda_args
		args = []
		type = :required
		loop do
			if splatsplat
				name = identifier or error 'splatsplat without an identifier'
				args.push Ast::Lambda::SplatSplat.new(name)
				break
			elsif splat
				error 'unexpected splat' unless type == :required || type == :optional
				name = identifier or error 'splat without an identifier'
				args.push Ast::Lambda::Splat.new(name)
				type = :keyword
			else
				name = identifier || break
				if token(:op, '=')
					error 'unexpected optional argument' unless type == :required || type == :optional
					type = :optional
					default = expression or error 'expected an expression for optional argument'
					args.push Ast::Lambda::Optional.new(name, default)
				elsif colon
					type = :keyword
					if (default = expression)
						args.push Ast::Lambda::KwargOptional.new(name, default)
					else
						args.push Ast::Lambda::Kwarg.new(name)
					end
				elsif type == :required
					args.push Ast::Lambda::Required.new(name)
				else
					error 'unexpected positional identifier'
				end
			end

			comma or break
		end
		args
	end

	def error(msg)
		raise RuntimeError, msg, caller(1)
	end

	def put_back(tkn)
		@peeked.push tkn
	end

	def satisfy
		begin
			@peeked.push @lexer.next if @peeked.empty?
		rescue StopIteration
			return
		end

		return unless yield @peeked.last
		@peeked.pop
	end

	def token(type=nil, value=nil, ignore_soft: true)
		nil while satisfy { _1.type == :endline && _1.value == :soft } if ignore_soft

		satisfy { 
			(type.nil? || _1.type == type) && (value.nil? || _1.value == value) 
		}
	end

	def text
		tkn = token(:text) and Ast::Text.new tkn.value
	end

	def number
		tkn = token(:number) and Ast::Number.new tkn.value
	end

	def identifier
		tkn = token(:ident) and Ast::Identifier.new tkn.value
	end

	def variable
		tkn = token(:ident) and Ast::Variable.new tkn.value
	end

	def regex
		tkn = token(:regex) and Ast::Regex.new tkn.value
	end

	def stackframe
		tkn = token(:stackframe) and Ast::Stackframe.new tkn.value
	end

	def lparen; token(:paren, '(') end
	def rparen; token(:paren, ')') end
	def lbrace; token(:paren, '{') end
	def rbrace; token(:paren, '}') end
	def lbracket; token(:paren, '[') end
	def rbracket; token(:paren, ']') end
	def splat; token(:op, '*') end
	def splatsplat; token(:op, '**') end
	def comma; token(:comma) end
	def colon; token(:colon) end
	def dot; token(:dot) end
	def lambda_args_start; token(:op, '|') end

	def assignment_op
		satisfy { _1.type == :op && _1.value.match?(/\A([-+*\/%&|^]|\*\*|>>|<<)?=\z/)}
	end

	alias lambda_args_end lambda_args_start
end

__END__
Parser.new(Lexer.from_string(<<EOS)).top.pprint rescue puts $!.to_s
disp(
	1,
	*a,
	b:,
	c: 3,
	('d'.'e'): 4,
	**f
);
EOS
__END__
parser = Parser.new Lexer.from_string(<<'=end')
foo = {
	|     # arguments start
	a,    # required positional argument,
	b=1,  # optional positional argument
	*c,   # all the remaining positional arguments
	d:,   # required keyword argument
	e: 4, # optional keyword argument
	**f   # all the remaining keyword arguments
	|     # arguments end

	a.(b) += # dynamically calculate fields
		[                # Map literal because of `key: value`
			c: [*c, 5],    # `[*c, 5]` is an array literal because no `:`
			d:,            # same as `d: d`
			(e): [:],      # dynamically determine the key. (`[:]` is an empty Map literal)
			**f            # use the remaining values
		]
	# a # return `a`
};
=end

# parser = Parser.new Lexer.from_string("a = b = c")
parser.top.pprint1.'
'
