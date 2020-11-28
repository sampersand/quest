

class Context
	def initialize
		@retreg = nil
		@instructions = []
	end


# class Register
# 	def initialize(num)
# 		raise TypeError, "register isn't an int" unless @num.is_a? Integer
# 		raise ArgumentError, "register index out of bounds" unless (0..255).include? @num
# 		@num = num
# 		@used = false
# 	end

# 	def to_s; "%#@num" end
# 	def inspect; "#{self.class}(#@num, used=#@used)" end

# 	def to_i; @num end
# 	alias to_int num

# 	def acquire; used? ? raise("Can't acquire a register twice") : @used = true end
# 	def free; !used? ? raise("Can't free a register twice") : @used = false end
# 	def used?; @used end

# 	def emit(where)
# 		where.write @num
# 	end

# 	class Scratch < Register
# 		def initialize(num)
# 			raise ArgumentError, "scratch index is out of bounds" unless (0..63).include? num
# 			super
# 		end
# 	end

# 	class Scratch < Register
# 		def initialize(num)
# 			raise ArgumentError, "scratch index is out of bounds" unless (0..63).include? num
# 			super num + 0b00111111
# 		end
# 	end


# 	class InstructionPointer < self
# 		def initialize; super 0b10000000 end
# 		undef acquire
# 		undef free
# 		undef used?
# 	end

# 	class Preserved

# end

# ScratchRegister = Class.new Register
# PreservedRegister = Class.new Register


class Block
	attr_reader :name
	def initialize(name='<anon>')
		@name = name
		@instructions = []
	end

	class Label
		def initialize(context)
			@context = context
			@lineno = nil
		end

		def bind
			raise "Can't bind a label twice" if @lineno
			@lineno = context.next_lineno
			freeze
		end

		def emit(where)
			where.write [@lineno].pack 'Q>'
		end
	end

	def next_lineno
		@instructions.length
	end

	def new_context

	end

	def label
		Label.new(self)
	end

	def to_s
		"#@name:#{@instructions.each_with_index.map { "\n#{_2}: #{_1}" }.join}"
	end
end
