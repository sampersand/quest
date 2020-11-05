#![allow(unused)]
use quest_core::Object;

#[derive(Debug)]
pub enum ByteCode {

	// Add(Object, Object)
}

enum Object {
	Integer(i32),
	
}

fn bytecode_add(lhs: Object, rhs: Object) -> Object {
	if lhs.is_a::<Number>() {
		Object::new(lhs + rhs.downcast_)
	}
}


/*
Number.divides = (self, rhs) -> { (rhs % self) == 0 };

disp(3.divides(12));

=========

$0 = getattr 'foo', 'x'
$1 = getattr 'bar', 'y'
add $0, $1

# ==>




nop, load, store
call, ret, raise
cmp, jeq, jne, jgt, jge, jlt, jle, jmp       # jump ops
neg, add, sub, mul, div, mod, pow,           # math ops
not, and, or, xor, shl, shr                  # bitwise ops
getattr, callattr, setattr, delattr, hasattr # quest-specific

load, assign, dynamicassign, call, literal, func2obj, funcargs, return, raise
getattr, callattr, setattr, delattr, hasattr
neg, pos, add, sub, mul, div, mod, pow,
bnot, band, bor, bxor, shl, shr,
eql, neq, lth, leq, gth, gteq, cmp


func0:
.0 = load 'Number'
.1 = literal.Text 'divides'
.2 = func2obj func1
.3 = setattr .0, .1, .2

.4  = getattr 'disp'
.5  = literal.Number 3
.6  = literal.Number 12
.7  = literal.Text 'divides'
.8  = callattr .5, .7, [.6]
.9  = call .4, [.8]

.10 = return 0

func1:
.0 = funcargs 'self', 'rhs'
.1 = load 'self'
.2 = load 'rhs'
.3 = mod .1, .2
.4 = literal.Number, 0
.5 = eql .3, .4
.6 = return .5

*/
