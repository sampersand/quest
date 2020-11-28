require_relative 'instruction'
class Label
	def initialize(context)
		@context = context
	end

	def bind
		raise "Can't bind a label twice" if defined? @lineno
		@lineno = context.next_lineno
		freeze
	end

	# def emit(where) # this will have to be relative to the instruction that uses it.
	# 	where.write [@lineno].pack 'Q>'
	# end
end

# class Register
# 	def initialize(context, number=nil)
# 		@context = context
# 		@number = number
# 	end

# 	def bind

class Context
	attr_reader :code
	protected :code

	def initialize
		@code = []
		@data = {}
		@temp_reg_num = 0
	end

	def merge(child)
		@code += child.code
	end

	def group(&block)
		instance_eval(&block)
		retreg
	end

	def child_eval(&block)
		child = Context.new
		$RETLVL += 1
		ret = child.group(&block)
		$RETLVL -= 1
		@code += child.code
		ret
	end

	def to_s
		@code.join "\n"
	end

	def emit(...)
		@code.each { _1.emit(...) }
	end

	def label; Label.new self end
	def tempreg; "tmp#{@temp_reg_num += 1}" end
	def retreg; "ret#$RETLVL" end
	def literal(val) val.inspect end

	def nop; @code.push Instruction::Nop.new end
	def storei(dst, val) @code.push Instruction::StoreI.new dst, val end
	def storel(dst, lit) @code.push Instruction::StoreL.new dst, lit end


	def loadl(dst, src) @code.push Instruction::LoadL.new dst, src end
	def mov(dst, src) @code.push Instruction::Mov.new dst, src end
	def push(reg) @code.push Instruction::Push.new reg end
	def pop(reg) @code.push Instruction::Pop.new reg end
	def pushb(byte) @code.push Instruction::PushByte.new byte end

	def getattrl(dst, obj, key) @code.push Instruction::GetAttrL.new dst, obj, key end
	def delattrl(dst, obj, key) @code.push Instruction::DelAttrL.new dst, obj, key end
	def setattrl(dst, obj, key, val) @code.push Instruction::SetAttrL.new dst, obj, key, val end
	def callattrl(dst, obj, key, argc, kwargc) @code.push Instruction::CallAttrL.new dst, obj, key, argc, kwargc end
	def getattr(dst, obj, key) @code.push Instruction::GetAttr.new dst, obj, key end
	def setattr(dst, obj, key, val) @code.push Instruction::SetAttr.new dst, obj, key, val end
	def delattr(dst, obj, key) @code.push Instruction::DelAttr.new dst, obj, key end
	def callattr(dst, obj, key, argc, kwargc) @code.push Instruction::CallAttr.new dst, obj, key, argc, kwargc end

	def call(offset) @code.push Instruction::Call.new offset end # TODO: maybe make this more dynamic
	def ret; @code.push Instruction::Ret.new end
	def dec(reg) @code.push Instruction::Dec.new reg end
	def inc(reg) @code.push Instruction::Inc.new reg end
	def newscope; @code.push Instruction::NewScope.new end
	def exitscope; @code.push Instruction::ExitScope.new end
	def loadscope(reg, layers_up=0); @code.push Instruction::LoadScope.new reg, layers_up end

	def cmp(reg) @code.push Instruction::Cmp.new reg end
	def jeq(offset) @code.push Instruction::Jeq.new offset end
	def jlt(offset) @code.push Instruction::Jlt.new offset end
	def jle(offset) @code.push Instruction::Jle.new offset end
	def jgt(offset) @code.push Instruction::Jgt.new offset end
	def jge(offset) @code.push Instruction::Jge.new offset end
	def jne(offset) @code.push Instruction::Jne.new offset end
	def jmp(offset) @code.push Instruction::Jmp.new offset end

	def neg(reg) @code.push Instruction::Neg.new reg end
	def add(dst, src) @code.push Instruction::Add.new dst, src end
	def sub(dst, src) @code.push Instruction::Sub.new dst, src end
	def mul(dst, src) @code.push Instruction::Mul.new dst, src end
	def div(dst, src) @code.push Instruction::Div.new dst, src end
	def mod(dst, src) @code.push Instruction::Mod.new dst, src end
	def pow(dst, src) @code.push Instruction::Pow.new dst, src end

	def lnot(reg) @code.push Instruction::Not.new reg end
	def land(dst, src) @code.push Instruction::And.new dst, src end
	def lor(dst, src) @code.push Instruction::Or.new dst, src end
	def xor(dst, src) @code.push Instruction::Xor.new dst, src end
	def shl(dst, src) @code.push Instruction::Shl.new dst, src end
	def shr(dst, src) @code.push Instruction::Shr.new dst, src end

	# Element Creation
	def poplist(dst, count) @code.push Instruction::PopList.new dst, count end
	def popmap(dst, count) @code.push Instruction::PopMap.new dst, count end
	def poptext(dst, count) @code.push Instruction::PopText.new dst, count end
	def popregex(dst, count) @code.push Instruction::PopRegex.new dst, count end
end

require_relative 'parser'

context = Context.new

$RETLVL = 0
parser = Parser.new Lexer.from_string <<EOS
bar = 'baz';
foo = [1, 2];
foo.(bar) = 3;
disp('foo=', foo, endl: '');
EOS
parser.top.generate_bytecode context
puts context

__END__
loadscope ret1, 0
mov tmp1, ret1
storei ret1, 3
setattrl tmp1, x, ret1
loadscope ret2, disp
getattrl ret2, "disp"
mov tmp2, ret1
loadscope ret3, x
getattrl ret3, "x"
mov ret1, ret2
storei ret2, 4
mov tmp1, ret2
add ret1, tmp1
push ret1
pushbyte 101
pushbyte 110
pushbyte 100
pushbyte 108
poptext ret1, 4
push ret1
poptext ret1, 0
push ret1
callattrl tmp2, (), 1, 1
[Finished in 0.3s]
