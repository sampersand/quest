Io.File('expression.qs').read().eval();

env = Environment();

#Parser(Io.File('knight.sq').read()).run(env)
#__EOF__##
#Parser(Io.File('examples.sq').read()).run(env);
#quit();
Parser('
func fizzBuzz (max) {
	i = 1;

	while i < max {
		if (i % 3) == 0 {
			print("Fizz")
		}

		if (i % 5) == 0 {
			print("Buzz")
		}

		if ((i % 3) * (i % 5)) != 0 {
			print(i)
		}

		i += 1

		print()
	}
}

fizzBuzz(20)
').run(env);
