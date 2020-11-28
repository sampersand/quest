class Instruction
	##### General  #####
	class << self
		def new_instruction(tag, cls, **args)
			emit_arr = '[TAG'
			emit_str = 'C'

			args.each do |key, type|
				emit_arr << ", #{key}.to_i"
				case type
				when :reg     then emit_str << 'C'
				when :byte    then emit_str << 'C'
				when :i64     then emit_str << 'q>'
				when :u64     then emit_str << 'Q>'
				when :offset  then emit_str << 'q>'
				when :literal then "<uh oh>"
				else raise "unknown arg type: #{type}"
				end
			end
			emit_arr << ']'
			args = args.keys

			class_eval <<~RB
				class #{cls} < Instruction
					TAG=#{tag}

					attr_reader #{args.map(&:inspect).join ', '}

					def initialize(#{args.join ', '}) 
						#{args.map{ "@#{_1} = #{_1}.freeze;" }.join}
						freeze
					end

					def to_s
						"#{cls.downcase}#{args.empty? ? '' : ' '}#{args.map { "\#{#{_1}}" }.join ', '}"
					end

					def inspect
						"#{cls}(#{args.map { "\#{#{_1}.inspect}" }.join ', ' })"
					end

					def emit(where)
						where.write #{emit_arr}.pack '#{emit_str}'
					end
				end
			RB
		end
	end

	# General
	new_instruction 0b00000000, :Nop
	new_instruction 0b00000001, :StoreI, dst: :reg, val: :i64
	new_instruction 0b00000010, :StoreL, dst: :reg, lit: :literal
	new_instruction 0b00000011, :LoadL, dst: :reg, src: :literal
	new_instruction 0b00000100, :Mov, dst: :reg, src: :reg
	new_instruction 0b00000101, :Push, src: :reg
	new_instruction 0b00000110, :Pop, src: :reg
	new_instruction 0b00000111, :PushByte, val: :byte

	# Quest-Specific
	new_instruction 0b00001000, :GetAttrL, dst: :reg, obj: :reg, key: :literal
	new_instruction 0b00001001, :DelAttrL, dst: :reg, obj: :reg, key: :literal
	new_instruction 0b00001010, :SetAttrL, dst: :reg, obj: :reg, key: :literal, val: :reg
	new_instruction 0b00001011, :CallAttrL, dst: :reg, obj: :reg, key: :literal, args: :u64, kwargs: :u64
	new_instruction 0b00001100, :GetAttr, dst: :reg, obj: :reg, key: :reg
	new_instruction 0b00001101, :SetAttr, dst: :reg, obj: :reg, key: :reg, val: :reg
	new_instruction 0b00001110, :DelAttr, dst: :reg, obj: :reg, key: :reg
	new_instruction 0b00001111, :CallAttr, dst: :reg, obj: :reg, key: :reg, args: :u64, kwargs: :u64

	# Control Flow
	new_instruction 0b00010000, :Call, dst: :offset
	new_instruction 0b00010001, :Ret
#	new_instruction 0b00010010, :Raise, dst: :reg
	new_instruction 0b00010011, :Dec, reg: :reg
	new_instruction 0b00010100, :Inc, reg: :reg
	new_instruction 0b00010101, :NewScope
	new_instruction 0b00010110, :ExitScope
	new_instruction 0b00010111, :LoadScope, reg: :reg, amnt: :u64
	#                      +-0
	new_instruction 0b00011000, :Cmp, reg: :reg
	new_instruction 0b00011001, :Jeq, to: :offset
	new_instruction 0b00011010, :Jlt, to: :offset
	new_instruction 0b00011011, :Jle, to: :offset
	new_instruction 0b00011100, :Jgt, to: :offset
	new_instruction 0b00011101, :Jge, to: :offset
	new_instruction 0b00011110, :Jne, to: :offset
	new_instruction 0b00011111, :Jmp, to: :offset

	# Math
	new_instruction 0b00100000, :Neg, reg: :reg
	new_instruction 0b00100001, :Add, dst: :reg, src: :reg
	new_instruction 0b00100010, :Sub, dst: :reg, src: :reg
	new_instruction 0b00100011, :Mul, dst: :reg, src: :reg
	new_instruction 0b00100100, :Div, dst: :reg, src: :reg
	new_instruction 0b00100101, :Mod, dst: :reg, src: :reg
	new_instruction 0b00100110, :Pow, dst: :reg, src: :reg

	# Bitwise
	new_instruction 0b00100111, :Not, reg: :reg
	new_instruction 0b00101000, :And, dst: :reg, src: :reg
	new_instruction 0b00101001,  :Or, dst: :reg, src: :reg
	new_instruction 0b00101010, :Xor, dst: :reg, src: :reg
	new_instruction 0b00101011, :Shl, dst: :reg, src: :reg
	new_instruction 0b00101100, :Shr, dst: :reg, src: :reg

	# Type creation
	new_instruction 0b00110000, :PopList, dst: :reg, count: :u64
	new_instruction 0b00110001, :PopMap, dst: :reg, count: :u64
	new_instruction 0b00110010, :PopText, dst: :reg, count: :u64
	new_instruction 0b00110011, :PopRegex, dst: :reg, count: :u64
end
