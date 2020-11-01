Io.File('expression.qs').read().eval();

env = Environment();

Parser(Io.File('knight.sj').read()).run(env)
