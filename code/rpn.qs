$calc_rpn = {
	$stack = [];
	$rpn = _1;
	$curr = '';
	while($rpn, {
		$c = rpn.$shift();
		if(('0' <= c) & (c <= '9') | (c == '.'), {
			curr.$push(c);
		}, {
			if(curr, {
				stack.$push(curr.$@num());
				curr.$clear();
			});

			if((c == '+') | (c == '-') | (c == '*') | (c == '/'), {
				$rhs = stack.$pop();
				$lhs = stack.$pop();
				stack.$push(lhs.c(rhs));
			});
		})();
	});

	stack.$pop()
};

disp(calc_rpn("12 24 - 12.4 *"));