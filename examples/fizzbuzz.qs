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
1.$upto(100)
	.$map({
		15.$divides(_0).$and(:0.$return << 'FizzBuzz');
		3.$divides(_0).$and(:0.$return << 'Fizz');
		5.$divides(_0).$and(:0.$return << 'Buzz');
		_0
	})
	.$each(disp);

# $fizzbuzz = {
# 	1.$upto(_0)
# 		.$map({
# 			(_0 % 3).$if(
# 				{ (_0 % 5).$if({ _0 }, { "Buzz" }) },
# 				{ (_0 % 5).$if({ "Fizz" }, { "FizzBuzz" }) }
# 			)
# 		})
# };

# fizzbuzz(100).$each(disp);

# # Tests
# assert([
# 	1, 2, "Fizz", 4, "Buzz", "Fizz", 7, 8, "Fizz", "Buzz", 11, "Fizz", 13, 14,
# 	"FizzBuzz", 16, 17, "Fizz", 19, "Buzz", "Fizz", 22, 23, "Fizz", "Buzz", 26,
# 	"Fizz", 28, 29, "FizzBuzz"
# ] == fizzbuzz(30));
