fibonacci = {
	if (_0 <= 1, { _0 }, /* else */ {
		fibonacci(_0 - 1) + fibonacci(_0 - 2)
	})
};

print(fibonacci(10));
# print(10.fibonacci());
