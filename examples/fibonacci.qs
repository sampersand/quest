# A Fibonacci function that keeps an internal memo of previously computed
# values.
# 
# Note that we immediately call this block after its definition: `fibonacci` is
# set to the return value of the block.
Number.$fibonacci = {
	# initialize memo to a blank object.
	$memo = { __this__ }();

	# Then assign some starting, initial values.
	memo.0 = 0;
	memo.1 = 1;

	# This is the "actual" fibonacci function that will be run when `fibonacci
	# is called. We still have access to the enclosing scope, which allows us to
	# hide the `memo` object so no one else can interact with it.
	{
		if(memo.$__has_attr__(_0), {
			memo._0
		}, {
			disp("memoizing:", _0);
			memo._0 = (_0 - 1).$fibonacci() + (_0 - 2).$fibonacci()
		})
	}
}();

disp(5.$fibonacci());
disp(10.$fibonacci());

/* => 
memoizing: 5
memoizing: 4
memoizing: 3
memoizing: 2
5
memoizing: 10
memoizing: 9
memoizing: 8
memoizing: 7
memoizing: 6
55
*/

# Tests
assert(5.$fibonacci() == 5);
assert(10.$fibonacci() == 55);
