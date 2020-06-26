# TODO

- Cleaning up of hacky code
- Finish implementation of builtin objects
- Change `EqResult` to be only for `Key`s
- Implementing Copy-on-Write for object mappings?

```quest
$x = {
	$a = true;
	disp(a);
	a &= false;
};

x(); # => true
x(); # => false
```