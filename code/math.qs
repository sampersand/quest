# $fns = [$sin, $cos, $tan, $acos, $asin, $atan];

# while({ fns }, {
# 	$fn = fns.$pop();
# 	Number.fn = {
# 		system('ruby', '-e', 'puts Math.' + fn + ' ' + _0).$@num()
# 	};
# });

# $PI  = Number.$PI;
# $E   = Number.$E;
# $INF = Number.$INF;
# $NAN = Number.$NAN;

# $Math = {

	$sin = { system('ruby', '-e', 'puts Math.sin ' + _1).$@num() };
	$cos = { system('ruby', '-e', 'puts Math.cos ' + _1).$@num() };
	$tan = { system('ruby', '-e', 'puts Math.tan ' + _1).$@num() };
	$asin = { system('ruby', '-e', 'puts Math.asin ' + _1).$@num() };
	$acos = { system('ruby', '-e', 'puts Math.acos ' + _1).$@num() };
	$atan = { system('ruby', '-e', 'puts Math.atan ' + _1).$@num() };
	
	$PI  = Number.$PI;
	$E   = Number.$E;
	$INF = Number.$INF;
	$NAN = Number.$NAN;

	__this__
# }();