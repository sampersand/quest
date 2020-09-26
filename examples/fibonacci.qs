# A fibonacci function that keeps an internal memo of previously computed values.
# 
# Note that we immediately call this block after its definition: `fibonacci` is
# set to the return value of the block.
$fibonacci = {
	# initialize memo to a blank object.
	$memo = { __this__ }();
	# then assign some values to it.
	memo.0 = 0;
	memo.1 = 1;

	# This is the "actual" fibonacci function that will be run when `fibonacci` is called.
	{
		if(memo.$__has_attr__(_1), {
			memo._1
		}, {
			disp("memoizing:", _1);
			memo._1 = fibonacci(_1 - 1) + fibonacci(_1 - 2)
		})
	}
}();

disp(fibonacci(5));
disp(fibonacci(10));

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
assert(fibonacci(5) == 5);
assert(fibonacci(10) == 55);
