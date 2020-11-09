class Token
	attr_reader :value

	def initialize(value)
		@value = value.freeze
		freeze
	end

	def inspect
		"#{self.class}(#{@value.inspect})"
	end

	class Number < Token
		def self.parse(input)
		end
	end
end
aq
	Number = Class.new Token
	Identifier = Class.new Token
	String = Class.new Token
end


p Token::Number.new 'a'
