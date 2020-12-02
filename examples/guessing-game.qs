# A guessing game.

# If an argument was supplied to this executable, use that as the maximum value.
# otherwise, default to 100.
MAX = if(:0.__has_attr__('_1'), { _1.@num() }, { 100 });

disp('Guessing game! Guess from 1-' + MAX);

# Because `rand` returns a non-whole number, we need to convert it to one.
secret = rand(1, MAX + 1).floor();

guesses = 0;

# `loop` is a synonym for `while true`
loop({
	# Add one to the amount of guesses.
	guesses += 1;

	# print "> " out, prompt for a value, and then convert that value to a number.
	guess = prompt("> ").@num();

	if (guess == secret) {
		disp("perfect!\nit took you", guesses, "guesses");
		quit(0);
	};

	# use `ifl`, which is the ternary operator for literal values.
	disp("too ", ifl(guess > secret, "high", "low");
});
