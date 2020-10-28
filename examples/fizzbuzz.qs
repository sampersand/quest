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

Number.$divides = { (_1 % _0) == 0 };

$fizzbuzz = $max -> {
	1.$upto(max)
		.$map($n -> {
			15.$divides(n).$then('FizzBuzz'.$return);
			3.$divides(n).$then('Fizz'.$return);
			5.$divides(n).$then('Buzz'.$return);
			n
		})
};

fizzbuzz(100).$each(disp);

# Tests
assert([
	1, 2, "Fizz", 4, "Buzz", "Fizz", 7, 8, "Fizz", "Buzz", 11, "Fizz", 13, 14,
	"FizzBuzz", 16, 17, "Fizz", 19, "Buzz", "Fizz", 22, 23, "Fizz", "Buzz", 26,
	"Fizz", 28, 29, "FizzBuzz"
] == fizzbuzz(30));
