fibonacci = {
	if (_0 <= 1, { _0 }, /* else */ {
		fibonacci(_0 - 1) + fibonacci(_0 - 2)
	})
};

print(fibonacci(30));
# print(10.fibonacci());
