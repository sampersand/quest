# initial
x = 0;
while ({ x < 10 }) {
	print(x);
	x += 1
};

# without the `(){}` sugar
x = 0;
while({ x < 10 }, {
  print(x);
  x += 1
});

# all operators are really just sugar for method calls
'x'.'='(0);
while({ x.'<'(10) }, {
	print(x);
	x.'+='(1)
});

# all variables are actually executing a string of that variable
'x'.'='(0);
'while'()({ 'x'().'<'(10) }, {
  'print'()('x'());
  'x'().'+='(1)
});

# `;` is an operator
'x'.'='(0).';'(
	'while'()({ 'x'().'<'(10) }, {
		'print'()('x'()).';'(
			'x'().'+='(1)
		)
	})
)
