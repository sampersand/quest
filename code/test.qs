"Person" = {
	"()" = {
		$__parent__ = Person;
		$first = _1;
		$last = _2;
		disp(__id__);
		__this__
	};

	"@text" = {
		disp(_1::'__id__');
		_1.first + " " + _1.last
	};

	__this__
}();

disp(Person("sam", "w"));


# (class::'__id__') == (((class."()")())::'__parent__')::'__id__'

# "class" = {
# 	"name" = "class";
# 	"()" = { __this__ };
# 	# disp("__id__=", __id__, ", __this__::()::__id__=", (__this__::'()')::'__id__');
# 	__this__
# }();

# (class::'()')(class)
# # ((class::'()')(class)), class, __this__
# # $Frac = {
# # 	"@text" = "frac";
# # 	"return_self" = {
# # 		# disp(__id__);
# # 		# __parent__::'__id__',
# # 		(__this__::"@text")(__this__)
# # 	};
# # 	disp("\t\tinside frac: __id__=", __id__,
# # 		", __this__.__id__=", (__this__::'__id__'),
# # 		", return_self.__id__=", return_self::'__id__'
# # 	);

# # 	__this__
# # }();

# # (Frac::'return_self')(Frac)
# # $bound_ret_self = Frac.'return_self';
# # # disp(Frac::'__id__', ' ', bound_ret_self())
# # disp("\t\tFrac.__id__=", Frac::'__id__', ", bound_ret_self.__id__=", bound_ret_self::'__id__');
# # disp("\t\t", (bound_ret_self::'__bound_object__')::'__id__');
# # disp("\t\t", (bound_ret_self::'__bound_object_owner__')::'__id__');
# # bound_ret_self()

# # disp((Frac.'return_self')())
# # disp((1 == 1).'__id__');
# # # $x = { $a = 0; '[]' = { disp('hi'); }; __this__ }();

# # # while(null, { ((x.'a') <=> 10) != 1 }, { disp("x=" + x.'a'); x.'a' = ((x.'a') + 1); });

# # disp(1.'__id__');
# # disp(1.'__id__');
# # # disp(x[12])