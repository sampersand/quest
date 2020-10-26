Number.$upto = {
	$arr = [];
	$start = _0.$clone();
	$stop = _1;

	while({ start <= stop }, {
		arr.$push(start.$clone());
		start += 1;
	});

	arr
};

$fizzbuzz = {
	1.$upto(_0)
		.$map({
			if(_0 % 3, {
				if(_0 % 5, $_0, {"Buzz"})
			}, {
				if(_0 % 5, {"Fizz"}, {"FizzBuzz"})
			})
		})
};

fizzbuzz(100).$each(disp);

# Tests
assert([
	1, 2, "Fizz", 4, "Buzz", "Fizz", 7, 8, "Fizz", "Buzz", 11, "Fizz", 13, 14,
	"FizzBuzz", 16, 17, "Fizz", 19, "Buzz", "Fizz", 22, 23, "Fizz", "Buzz", 26,
	"Fizz", 28, 29, "FizzBuzz"
] == fizzbuzz(30));
