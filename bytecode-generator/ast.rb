
class Struct
	alias inspectold inspect
	def pprint(where=$stdout, indent='')
		where.puts self.class.to_s.sub 'Ast::', ''
		mems = members
		mems.each do |name|
			last = name == mems.last
			where.write indent
			indent2 = indent
			if last
				where.write '└'
				indent2 += '   '
			else
				where.write '├'
				indent2 += '│  '
			end
			where.write "─#{name}: "
			case (member = public_send name)
			when Struct then member.pprint(where, indent2)
			when Array then where.puts; member.each_with_index do |mem, index|
				last = index == member.length - 1
				where.write indent2
				indent3 = indent2
				if last
					where.write '└'
					indent3 += '   '
				else
					where.write '├'
					indent3 += '│  '
				end
				where.write "─##{index}: "
				mem.pprint where, indent3
			end
			else where.puts member.inspect
			end
		end
	end
end

module Ast
	Identifier = Struct.new :value do
		def generate_bytecode(context) fail "generating bytecode for _identifier_s is undefined." end
	end

	Text = Struct.new :value do
		def generate_bytecode(context)
			value = value()
			context.group do
				value.each_byte do |b|
					pushb b
				end
				poptext retreg, value.bytes.length
			end
		end
	end

	Number = Struct.new :value do
		def generate_bytecode(context)
			# this will also fail for massive numbers, but let's ignore that edge case for now.
			unless value.is_a?(Integer)
				fail "todo: nonintegers"
			end

			value = value()
			context.group { storei retreg, value }
		end
	end

	Variable = Struct.new :value do
		def generate_bytecode(context)
			value = value()
			context.group do
				stackframe = child_eval { Stackframe::new(0).generate_bytecode(_1) }
				getattrl retreg, stackframe, literal(value)
			end
		end
	end

	Regex = Struct.new :value do 
		def generate_bytecode(context) fail "TODO: bytecode for Regex" end
	end

	Stackframe = Struct.new :value do 
		def generate_bytecode(context)
			value = value()
			context.group do
				loadscope retreg, value
			end
		end
	end

	Unary = Struct.new :op, :arg do
		def generate_bytecode(context)
			arg = arg()
			op = op()

			context.group do
				mov retreg, child_eval { arg.generate_bytecode(_1) }

				case op
				when '+' then callattrl retreg, retreg, '+@', 0, 0
				when '-' then neg retreg
				when '~' then bnot retreg
				when '!' then lnot retreg
				else fail "unknown unary op '#{op}"
				end
			end 
		end
	end

	Binary = Struct.new :op, :lhs, :rhs do
		def generate_bytecode(context)
			lhs = lhs()
			rhs = rhs()
			op = op()
			context.group do
				tmp = tempreg
				mov retreg, child_eval { lhs.generate_bytecode(_1) }

				if rhs.is_a? Identifier
					rhs = Text.new rhs.value
				end

				mov tmp, child_eval { rhs.generate_bytecode(_1) }

				case op
				when '+'  then  add retreg, tmp
				when '-'  then  sub retreg, tmp
				when '*'  then  mul retreg, tmp
				when '/'  then  div retreg, tmp
				when '%'  then  mod retreg, tmp
				when '**' then  pow retreg, tmp
				when '&'  then band retreg, tmp
				when '|'  then  bor retreg, tmp
				when '^'  then  xor retreg, tmp
				when '<<' then  shl retreg, tmp
				when '>>' then  shr retreg, tmp
				when '.', '::'

					push tmp
					callattrl retreg, retreg, op, 1, 0
				else fail "unknown binary op '#{op}'."
				end
			end
		end
	end

	Assignment = Struct.new :field, :value do
		def generate_bytecode(context)
			FieldAssignment.new(Stackframe.new(0), field, value).generate_bytecode(context)
		end
	end

	FieldAssignment = Struct.new :subject, :field, :value do
		def generate_bytecode(context)
			subject = subject()
			field = field()
			value = value()

			context.group do
				subj = tempreg
				mov subj, child_eval { subject.generate_bytecode(_1) }
				if field.is_a? Identifier
					setattrl retreg, subj, field.value, child_eval { value.generate_bytecode(_1) }
				else
					fld = tempreg
					mov fld, child_eval { field.generate_bytecode(_1) }
					setattr retreg, subj, fld, child_eval { value.generate_bytecode(_1) }
				end
			end
		end
	end

	ParenPhrase = Struct.new :body do
		def generate_bytecode(...)
			body.generate_bytecode(...)
		end
	end

	Statements = Struct.new :body do
		def generate_bytecode(context)
			body = body()
			context.group do 
				ret = nil
				body.each do |stmt|
					ret = stmt.generate_bytecode context
				end
				ret
			end
		end
	end


	List = Struct.new :items do
		def generate_bytecode(context)
			items = items()
			context.group do
				elements = 0
				items.each do |item|
					if item.is_a?(List::Item)
						push child_eval { item.item.generate_bytecode(_1) }
						elements += 1
						next
					end

					fail "unknown list item: #{item}" unless item.is_a? List::Splat
					fail "todo: list splat."
				end

				poplist retreg, elements
			end
		end
	end
	List::Splat = Struct.new :list
	List::Item = Struct.new :item

	Map = Struct.new :fields do
		def generate_bytecode(context)
			fields = fields()
			context.group do
				elements = 0
				fields.each do |field|
					if field.is_a?(Map::Field)
						push child_eval { field.key.generate_bytecode(_1) }
						push child_eval { field.value.generate_bytecode(_1) }
						elements += 1
						next
					end

					fail "unknown map field: #{field}" unless field.is_a? Map::SplatSplat
					fail "todo: map splat."
				end

				popmap retreg, elements
			end
		end
	end
	Map::SplatSplat = Struct.new :map
	Map::Field = Struct.new :key, :value

	Lambda = Struct.new :args, :body
	Lambda::Required      = Struct.new :name
	Lambda::Optional      = Struct.new :name, :default
	Lambda::Kwarg         = Struct.new :name
	Lambda::KwargOptional = Struct.new :name, :default
	Lambda::Splat         = Struct.new :name
	Lambda::SplatSplat    = Struct.new :name

	FunctionCall = Struct.new :func, :args, :kwargs do
		def generate_bytecode(context)
			func = func()
			args = args()
			kwargs = kwargs()
			argc = 0
			kwargc = 0
			context.group do
				funcreg = tempreg
				mov funcreg, child_eval { func.generate_bytecode(_1) }

				args.each do |arg|
					case arg
					when FunctionCall::Positional
						argc += 1
						push child_eval { arg.value.generate_bytecode(_1) }
					when FunctionCall::Splat then fail "todo"
					else fail "unknown arg type: #{arg}"
					end
				end

				kwargs.each do |arg|
					case arg
					when FunctionCall::Kwarg
						kwargc += 1
						push child_eval { arg.key.generate_bytecode(_1) }
						push child_eval { arg.value.generate_bytecode(_1) }
					when FunctionCall::SplatSplat then fail "todo"
					else fail "unknown kwarg type: #{arg}"
					end

				end

				callattrl retreg, funcreg, '()', args.length, kwargs.length
			end
		end
	end

	FunctionCall::Splat = Struct.new :list
	FunctionCall::Positional = Struct.new :value
	FunctionCall::Kwarg = Struct.new :key, :value
	FunctionCall::SplatSplat = Struct.new :map
end
