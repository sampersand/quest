$i = 10;
while($i, {
	__stack__.$get(2).$i = i - 1;
	disp(i);
});
# $x = 3;
# {
# 	$name = "a";
# 	{
# 		$name = "b";
# 		{
# 			$name = "c";
# 			disp(__stack__);
# 			disp(__stack__.$get(-1).$x = 4);
# 		}()
# 	}()
# }();

# disp(x);