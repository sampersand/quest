require 'stringio'

class Token
	attr_reader :type, :value
	def initialize(type, value=nil)
		@type, @value = type.freeze, value.freeze
		freeze
	end

	def inspect
		return type unless value
		"#{type}(#{value.inspect})"
	end

	alias to_s value

	def ==(rhs)
		@type == rhs.type && @value == rhs.value
	end
end

## A Lexer for Quest's tokens.
class Lexer
	class LexingError < RuntimeError
		attr_reader :lexer
		def initialize(msg, lexer)
			@lexer = lexer
			super msg
		end
	end

	include Enumerable

	# Create a new lexer from a file
	def self.from_file(file)
		new open(file, 'r'), file
	end

	# Create a new lexer from the string.
	def self.from_string(string, filename='<eval>')
		new StringIO.new(string), filename
	end

	attr_accessor :filename

	# Create a new lexer.
	def initialize(io, filename)
		@io = io
		@filename = filename
		@peeked = []
	end

	def close; @io.close end
	def eof?; @io.eof? && @peeked.empty? rescue true end

	def next
		each.next
	end

	def each
		return to_enum __method__ unless block_given?

		until eof?
			next if whitespace || comment
			# TODO: Regex & Stackframe literal
			yield misc || number || identifier || text || parens || operator || error("unrecognized char #{any.inspect}")
		end

		close
	end

	private

	def error(msg)
		err = LexingError.new(msg, self)
		err.set_backtrace caller 1
		raise err
	end

	def parens
		chr = char('(', ')', '[', ']', '{', '}') and Token.new(:paren, chr)
	end

	def operator
		Token.new(:op, case
			when char('<')
				case
				when char('=') then "<=#{char '>'}"
				when char('<') then "<<#{char '='}"
				else '<'
				end
			when char('>') then ">#{char '>'}#{char '='}"
			when char('=') then "=#{char '='}"
			when char('!') then '!'
			when char('&') then "&#{char '='}"
			when char('|') then "|#{char '='}"
			when char('^') then "^#{char '='}"
			when char('+') then "+#{char '='}"
			when char('-') then "-#{char '='}"
			when char('*') then "*#{char '*'}#{char '='}"
			when char('/') then "/#{char '='}"
			when char('%') then "%#{char '='}"
			when char('~') then '~'
			else return
			end
		)
	end

	def misc
		case
		when endline   then Token.new(:endline, :soft)
		when char(';') then Token.new(:endline, :hard)
		when char(',') then Token.new(:comma)
		when char('.') then Token.new(:dot)
		when char(':') then char(':') ? Token.new(:coloncolon) : Token.new(:colon)
		end
	end

	def number
		integer_radix || float
	end

	def integer_radix; return end # TODO
	def float
		num = digits or return

		is_float = false
		if char('.')
			is_float = true
			num += '.'
			num += digits || (unget '.'; return Token.new(:number, num))
		end

		if char('e', 'E')
			is_float = true
			num += 'e' + (char('+', '-') || '') + (digits or error 'missing mantissa')
		end

		Token.new :number, is_float ? num.to_f : num.to_i
	end

	def digits
		(digit or return) + accumulate { char('_') ? redo : digit }
	end

	def identifier
		start = alpha || char('@') or return
		Token.new(:ident, start + accumulate { alphanum })
	end

	def text
		contents = single_quoted_text || double_quoted_text or return
		Token.new(:text, contents)
	end

	def single_quoted_text
		char("'") && accumulate { any unless char("'") }
	end

	def double_quoted_text
		char('"') && accumulate {
			next if char '"'
			next any unless char '\\'

			case chr = any
			when '\\', "'", '"' then chr
			when 't' then "\t"
			when 'f' then "\f"
			when 'r' then "\r"
			when '0' then "\0"
			when 'x'
				h1 = hexdigit or error "bad hex escape"
				h2 = hexdigit or error "bad hex escape"
				[(h1.to_i(16) << 4) + h2.to_i(16)].chr
			when 'u'
				char '{' and raise "TODO: text escapes with '{"
				h1 = hexdigit or error "bad hex escape"
				h2 = hexdigit or error "bad hex escape"
				h3 = hexdigit or error "bad hex escape"
				h4 = hexdigit or error "bad hex escape"
			when 'U' then raise "TODO: text escapes with 'U'"
			else error "unknown text escape '#{chr}'"
			end
		}
	end

	def accumulate
		acc = ""
		while (last = yield)
			acc += last
		end
		acc
	end

	def comment
		char('#') && accumulate { any unless endline }
	end

	def unget(chr)
		@peeked.push chr
	end

	def satisfy
		@peeked.push @io.getc || return if @peeked.empty?

		@peeked.pop if yield @peeked.last
	end

	def any
		satisfy { true }
	end

	def char(*chrs)
		satisfy(&chrs.method(:any?))
	end

	def digit
		satisfy { _1.match? /\d/ }
	end

	def hexdigit
		satisfy { _1.match? /\h/ }
	end

	def whitespace
		satisfy { _1.match?(/\s/) && !_1.match?("\n") } # `\n` is a soft endline
	end

	def alpha
		satisfy { _1.match? /[a-zA-Z_]/ }
	end

	def alphanum
		alpha || digit
	end

	def endline
		char "\n"
	end
end
