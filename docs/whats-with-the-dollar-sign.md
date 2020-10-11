# What's the deal with `$`?
If you've looked at some Quest code, you're bound to have run into `$`. At first glance, however, it's not used consistently:
```quest
$where_backwards = "dlrow";
disp("Hello", where_backwards.$reverse()); # => Hello world
```
What's the deal? Why do identifiers sometimes have `$` and sometimes they don't?

## Dollar Signs and Strings
To put it simply, Quest doesn't have "identifiers" in the traditional sense: Instead, it uses `Text`s where most other languages would stereotypically use identifiers. In fact, `$foo` is _identical_ to `"foo"` (they both parse into a `Text` primitive). The dollar-sign syntax is simply added as a way to improve readability. We can technically rewrite the original example with _just_ `$`:
```quest
$where_backwards = $dlrow;
disp($Hello, where_backwards.$reverse());
```
... or with strings too:
```quest
"where_backwards" = "dlrow";
disp("Hello", where_backwards."reverse"());
```
One thing to note is that not every `Text` can be written with `$`s. Only variable names or operators (including `()`) may be used. So while `$foo`, `$+` and `$()` are valid identifiers, something like `$0` is not

## Identifiers
You may be wondering, "but wait, if there's no identifiers what's `disp`?" It clearly _looks_ like an identifier, and is used in places you'd normally expect one to be, so how is it not an identifier? Well, it's actually another special form of `Text`.

When you write `disp`, you're actually looking up the binding for the value `disp` in the current scope. That is, it's a shorthand for `:0."disp"` (`:0` means "the current scope"). The `$`-style `Text` and the raw identifier shorthands allow us to write succincter, cleaner, and easier-to-understand code. Imagine what a source file would be like if it was _just_ the longhand:
```
:0."where_backwards" = "dlrow";
:0."disp"("Hello", :0."where_backwards"."reverse"());
```
Eesh. But, this serves a purpose: It allows the language to be truly key-value based, as "identifiers" are actually objects!

# Why use Dollar Signs---The `.` operator
So we've now explained _how_ the `$` works. But why? Simply put, it allows us to treat identifiers/variable names as simply another object type---we're no longer limited to indexing via `Text`s, but we also can use `Number`s, `Boolean`s, or anything else! (You can read more about the rational behind this in the [Object and Maps](obejcts-and-maps-part1.md) series.)  This gives us a lot of flexibility in how we define and interact with objects.

In fact, `.` is an operator, just like `+` or `*`! Its normal function is just what you'd expect: Looking up values associated with keys. However, you can completely redefine it however you want, allowing you to redefine how attributes work. This Let's dive into the `.` operator!

## Strings as Identifiers
The most stereotypical use of the `.` operator (and its cousin `.=`) is via `$`-style `Text`s: 
```
# Create an empty "object".
$stoplights = {:0}();

stoplights.$green = "go!";
stoplights.$red = "stop!";

disp(stoplights.$green); # => go!
```
Because of this, a lot of optimization goes on behind the scenes to ensure that lookups on `Text` identifiers are fast. However, this is not the only use for `.`

## Accessing fields dynamically
Because `.` is an operator, it actually evaluates its operands before performing the `.` operation. (This is the reason why we use `Text`s as identifiers: We need to evaluate each side, and evaluating a raw identifier looks it up in the current scope.) This means we're not limited to indexing via literals. In this example, we're able to dynamically lookup names and nicknames:
```
$names = {:0}();

names.$william = "William";
names.$william_nickname = "Bill";
names.$richard = "Richard";
names.$richard_nickname = "Dick";

$name = "richard";

disp("The nickname of", names.name, "is:", names.(name + '_nickname'));
	# => The nickname of Richard is: Dick.
```
The ability to dynamically lookup values is extremely powerful and allows for a lot of flexibility when designing things such as memoization or helper functions:

```
# Define a method on `Basic`: most objects inherit from this, so will
# have this method in scope.
Basic.$get_or_default = {
	# `_0` is analogous to `this`/`self` in other languages,
	# `_1` is the first argument to the function, `_2` is the second, etc.
	if(_0.$__has_attr__(_1), { 
		_0._1
	}, {
		_2
	})
};

disp("foo".$get_or_default("nonexistent", 34))
	# => 34
```

## Overriding the `.` operator
You can also override `.` to provide custom implementations. Because of this, the `__get_attr__` method is supplied by `Pristine` (the parent which everything inherits from (by default) within Quest), although this can be overwritten itself (though probably shouldn't be.) For example, here's how you can provide array indexing via `.`:
```
$my_list = ["foo", "bar", "baz", "quux"];

# Here we override the `.` operator for `my_list`. Because we're overloading
# it, we need to make sure we don't accidentally call `my_list`'s `.` operator,
# as that would lead to infinite recursion. Instead, we use the `::` syntax to
# get an "unbound" version of them from `Pristine`. This allows us to specify
# which object we want to call this object on---in this case, `_0`.
my_list.$. = {
	if(Pristine::$__has_attr__(_0, _1), {
		Pristine::$.(_0, _1)
	}, {
		Pristine::$.(_0, $get)(_1)
	})
};

disp(my_list.2); # => baz
```

# Final Remarks
Anyways, hopefully this explains why you see `$` in Quest code (the `.` operator evaluates both sides eagerly) and the fun things you can do with the `.` operator.
