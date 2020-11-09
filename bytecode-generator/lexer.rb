## A Lexer for Quest's tokens.
class Lexer
	include Enumerable

	class << self
		def from_file(file)
			open file
		end

		def from_io(io)
			new io
		end
	end

	def initialize(input, )
end

require 'stringio'
Lexer.from_io StringIO.new("square = num -> { num ** 2 };\ndisp(num ** 2);")
