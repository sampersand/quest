# I currently haven't implemented a "read file" functionality yet...
$system('cat', './frac.qs').eval();
half = $Frac(1, 2);

$disp( half ); # => 1/2
$disp( half + 2 ); # => 5/2
$disp( half < 0.75 ) # => true