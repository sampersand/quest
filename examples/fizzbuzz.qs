Number.divides = (self, rhs) -> { (rhs % self) == 0 };
Number.divides = (factor, number) -> { (number % factor) == 0 };	

fizzbuzz = max -> {
	1.upto(max)
		.map(n -> {
			15.divides(n).then('FizzBuzz'.return);
			3.divides(n).then('Fizz'.return);
			5.divides(n).then('Buzz'.return);
			n
		})
};

fizzbuzz(100).each(print);

# Tests
assert([
	1, 2, "Fizz", 4, "Buzz", "Fizz", 7, 8, "Fizz", "Buzz", 11, "Fizz", 13, 14,
	"FizzBuzz", 16, 17, "Fizz", 19, "Buzz", "Fizz", 22, 23, "Fizz", "Buzz", 26,
	"Fizz", 28, 29, "FizzBuzz"
] == fizzbuzz(30));
