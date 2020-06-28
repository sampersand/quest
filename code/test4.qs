$i = 0;

loop {
	disp(i);
	i += 1;
	disp(__stack__);
	if(i > 5, {
		return(__stack__.$get(3))
	});
};

disp('done');
##__EOF__##

disp("-> a");
$a = {
	disp("\t-> b");
	$b = {
		disp("\t\t-> c");
		$c = {
			disp("\t\t\t-> d");
			$d = return(__stack__.$get(2), 2);
			disp("\t\t\t<- d:", d);
		}();
		disp("\t\t<- c:", c);
	}();
	disp("\t<- b:", b);
}();
disp("<- a:", a);
##__EOF__##

# $i = 10;
# while($i, {
# 	__stack__.$get(2).$i = i - 1;
# 	disp(i);
# 	return()
# });
$x = 3;
{
	$name = "a";
	{
		$name = "b";
		{
			$name = "c";
			disp(__stack__);
			disp(__stack__.$get(-1).$x = 4);
		}()
	}()
}();

disp(x);