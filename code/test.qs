Number.$sin = {
	# ask ruby to figure out the sin
	system('ruby', '-e', 'puts Math.sin(' + _1 + ')')
};

disp(2 << 64);
disp(12.$sin());

# $Math = {
# 	$cos = { system('ruby', '-e', 'puts Math.cos ' + _1) };
# 	__this__
# }();


# disp(Math::$cos(12));