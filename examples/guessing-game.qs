# A guessing game.

# If an argument was supplied to this executable, use that as the maximum value.
# otherwise, default to 100.
MAX = __args__.get(1).or(100).@num();

print('Guessing game! Guess from 1-' + MAX);

# Because `rand` returns a non-whole number, we need to convert it to one.
secret = rand(1, MAX + 1).floor();

guesses = 0;

# `loop` is a synonym for `while true`
loop {
	# Add one to the amount of guesses.
	guesses += 1;

	# print "> " out, prompt for a value, and then convert that value to a number.
	guess = prompt("> ").@num();

	if (guess == secret) {
		quit(0, "perfect!\nit took you" + guesses + "guesses");
	};

	# use `ifl`, which is the ternary operator for literal values.
	print("too ", ifl(guess > secret, "high", "low"));
}
