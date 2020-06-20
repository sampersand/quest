system('cat', 'code/frac3.qs').$eval();
$half = Frac(1, 2);
disp(half + half);

##__EOF__##

$frac = system('cat', 'code/frac3.qs').$eval({});
$half = frac.$Frac(1, 2);
disp(half + half);
