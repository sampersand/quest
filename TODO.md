# TODO

- Do we want a `returns` concept for `block`? couldn't the callee just ignore the return value?
- If a parent is set to `Pristine`, then `if` isn't accessible. Maybe add a `::` prefix operator that simply accesses something in the global scope? (such as `::$if(1, 2, 3)`)
- Cleaning up of hacky code
- Finish implementation of builtin objects
- Change `EqResult` to be only for `Key`s
- Implementing Copy-on-Write for object mappings?
- 
- Have boolean literals---having them as variables isn't working out, as shown below:
```quest
$x = {
	$a = true;
	disp(a);
	a &= false;
};

x(); # => true
x(); # => false
```