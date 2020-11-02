Io.File('expression.qs').read().eval();

env = Environment();

Parser(Io.File(_1).read()).run(env)
