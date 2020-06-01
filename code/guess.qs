# `_2` is the second argument from the environment, ie the first command line argument
$MAX = (_2.'@num')();
disp('Guessing game! Guess from 1-' + MAX);

$secret = (rand(1, MAX + 1).'floor')();
$guesses = 0;
$done = false;
$scope = __this__;

while({ !scope.'done' }, {
	$guess = (prompt('> ').'@num')();
	scope.'guesses' = guesses + 1;
	disp(
		if(($cmp = guess <=> secret) == -1, {
			'too low!'
		}, {
			if(cmp == 1, {
				'too high!'
			}, {
				scope.'done' = true;
				('perfect!\nit took you ' + guesses) + ' tr' + if(guesses == 1, 'y', 'ies')
			})()
		})()
	);
});