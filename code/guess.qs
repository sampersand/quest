$MAX = _1.$@num();
disp('Guessing game! Guess from 1-' + MAX);

$secret = rand(1, MAX + 1).$floor();
$guesses = 0;

while({ true }, {
	guesses += 1;

	$guess = prompt('> ').$@num();

	if(guess == secret, {
		disp('perfect!\nit took you', guesses, 'guesses');
		quit();
	}, {
		disp('too', if(guess > secret, { 'high' }, { 'low' }));
	})
});