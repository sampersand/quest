# Quest

[![CI](https://github.com/sampersand/quest/workflows/CI/badge.svg)](https://github.com/sampersand/quest/actions)

A language based around extensibility and freedom

# What's Quest
Quest is an "non-typed" language that is designed to allow for efficient code reuse. Similar to dynamically-typed languages in that types aren't relevant, Quest takes this a step further: There _are_ no types (just key-value pairs).

# Features
Quest supports everything you'd expect from a programming language and more!
- Simple, but powerful keyword-less syntax.
- Fundamentally based on hashmaps, not classes.
- Identifiers are first-class objects, just like everything else.
- Attributes and methods can be added to anything (including primitives!).
- _Everything_ is fair game, including methods defined on primitives.

# Installation
1. Clone the repo
2. If you haven't already, [install Rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
3. Run `$ cargo build` to create the project
4. `./quest [-h] [-f file] [-e script] [-- [args to pass to the quest program]]`
	- Command-line arguments are passed in the `__args__` method in the base script object.

If all arguments are omitted a REPL instance will be launched.

# Examples

See the `examples` folder for some examples of what Quest can do! Most of them expect that you've read at least

## Basic Examples
```php
# Text can either be single or double quotes: they're identical (like python).
where = "world";
disp("Hello, " + where + "!"); # => Hello, world!
```

Local variables are actually just string keys on the current object. The following is identical to the previous example:
```php
# `:0` is the same as `this` or `self` in other languages.
:0."where" = "world";
disp("Hello, " + :0."where" + "!"); # => Hello, world!
```

## Functions, Classes, and Maps

In Quest, there are no named/anonymous functions—they're both simply `Block`s, written as `{ ... }`:
```php
# Arguments are passed via local variables `_0`, `_1`, `_2`, etc.
# The last statement in a block is implicitly returned.
disp("4 squared is:", { _0 ** 2 }(4)); # => 4 squared is: 16

# You can assign anonymous functions to variables too. The `->` syntax
# can be used to name parameters.
square = n -> { n ** 2 };
disp("4 squared is:", square(4)); # => 4 squared is: 16

# You can even just straight-up add them to builtin classes:
# The `_0` argument is the the object that this method was called on, akin to
# `self` or `this` in other languages.
Number.square = n -> { n ** 2 };
disp("4 squared is:", 4.square());
```

Maps are created by simply returning the result of an executed block of code:
```php
traffic_lights = {
	# A blank scope is created whenever a block is called. Again, `:0` is the
	# same as `this` / `self` in other languages. 
	:0."red" = 'stop';
	:0."green" = "go";
	:0."yellow" = "go, if you can";

	:0 # Return the current scope
}();

disp("Green means", traffic_lights."green"); # => Green means go
```

**Classes are actually just objects too**: They're just a group of methods which are available for any object which makes the "class" its parent. There is no intrinsic concept of a "constructor" either, and is generally implemented by overloading the "call" (`()`) function and returning the new scope.
```php
Person = {
	# Whenever something is called, the `()` attribute is run.
	# "Constructors" are really just defining a `()` attribute that overwrites `__parents__` and
	# returns `:0` as the last value.
	'()' = (class, first, last) -> {
		# You can have multiple parents (to allow for multiple inheritance and mixins).
		# However, here we don't need to have multiple parents.
		__parents__ = [class];

		# The `first` and `last` variables are already defined in the current scope, so we don't
		# need to assign them!

		:0 # return the current scope, i.e. the new object
	};

	# Define the conversion to a text object
	@text = self -> { self.first + " " + self.last };

	:0 # like in `traffic_lights`, we return the current scope.
}();

person = Person("John", "Doe");
disp(person); # => "John Doe"
```

## No Keywords
Sticking to the theme of extensibility and freedom, there aren't traditional "keywords." Traditional control-flow keywords (such as `if`, `while`, and `return`) are simply attributes defined on the `Kernel` object (which most objects inherit from). And traditional "definition" keywords (such as `class` and function-declaration keywords) aren't relevant.

```php
factorial = n -> {
	# The if function executes whichever branch is chosen
	if(n <= 1, {
		1
	}, {
		n * factorial(n - 1)
	})
};
disp("10! =", factorial(10)); # => 10! = 3628800

i = 0;
while({ i < 5 }, {
	i += 1;
	disp("i =", i);
});
# => i = 1
# => i = 2
# => i = 3
# => i = 4
# => i = 5
```

### The `return` function
The `return` keyword is a bit different than other languages. Because there is no concept of "functions vs blocks", you must return to a specific scope:


```php
make_dinner = {
	the_magic_word = prompt("what's the magic word? ");
	if(the_magic_word != "please", {
		disp("You didn't say 'please'!");
		# `:0` is the current stackframe, `:1` is the stackframe above this
		# one in this case, that's the `make_dinner` stackframe. return `false`
		# from that stackframe.
		return(false, :1);
	});

	# Alternatively, you can use the shorthand of `false.return`, which returns
	# only a single level up.
	if(the_magic_word != "please", false.return);

	# Or even
	(the_magic_word == "please").else(false.return);

	collect_ingredients();
	prepare_stove();
	cook_food();
	set_table();

	disp("food's ready!");
	true # return `true`
};

# the `if` function can also be used as a ternary operator.
disp(if(make_dinner(), { "time to eat!" }, { "aww" }));
```

This also removes the need for `continue` and `break` keywords that so many other languages have:
```php
i = 0;
while({ i < 100 }, {
	i += 1;

	# Quest supports "truthy" values.
	if(i % 2, {
		# Return from the while loops's body's stackframe.
		# This is analogous to `continue`.
		return(:1);
	});

	disp("i =", i);

	if(i == 8, {
		disp("stopping.");
		# Return from the while loop's stackframe. 
		# This is analogous to `break`.
		return(:2);
	})
});

disp("done");

# => i = 2
# => i = 4
# => i = 6
# => i = 8
# => stopping
# => done
```


## No distinction between l- and r-values
(TODO: there's probably more I could do here to explain this better...)

Because there's no separate concept of an "identifier" in Quest (as all identifiers are really `Text`s), there's no _true_ l- or r-value concept. Instead, they are implemented via attributes defined on Text: `=` and `()`.

Unlike most languages, `=` is actually an _operator_. Only `Text` has it defined by default (but like any other operator, anything can overload it.):
```php
x = 5; # call the `Text::=` function implicitly
y.'='(6); # call the `Text::=` function explicitly

disp(:0.x, :0.y); # => 5 6

# now you can assign numbers.
# however, you can only access them via `:0.XXX`.
Number.'=' = Text::'=';

3 = 4;
disp(:0.3) # => 4
```

(Minor note: `a.b = c` doesn't actually use the `=` operator; it's syntactic sugar for the `.=` operator—`a.'.='(b,c)`—and is accessible on every object that inherits from `Pristine` (which is everything, by default).)

`Text` also has the `()` method defined, where it simply looks up its value in the current scope: (Bare variables, eg `foo`, were added so `'foo'()` wouldn't be necessary.)

```php
'x' = 5;
disp(x, 'x'(), :0.'x'); # => 5 5 5
```

## Everything is fair game
Most runtime languages support some form of instance variables that can be added to objects. However, Quest takes this a step further, and allows _everything_ to have attributes added/removed from them, including primitives like numbers. (For those language-savvy folks, every Quest object is a singleton object.)

```php
# define the `square` method on Numbers in general.
Number.square = self -> { self ** 2 };

twelve = 12;
disp(twelve.square()); # => 144

# define the `cube` method on this instance of 12.
twelve.cube = self -> { self ** 3 };
disp(twelve.cube); # => 1728

# no other `12` in the program has access to the `cube` method.
disp(12.__has_attr__('cube')); # => false
```

## More
See the `examples` folder for more examples of what Quest can do!

There's also some stuff I've written up in the `docs` that goes more into depth.

## TODO
I should probably add more discussion of Quest's features.

# Misc
## EBNF
```
PROGRAM := <block-inner>

expr
 := <primary>
  | UNARY_OP <expr>
  | <expr> BINARY_OP <expr>
  | <expr> <block> # Function call
 ;

primary := <block> | <literal>;
block := '(' <block-inner> ')' | '[' <block-inner> ']' | '{' <block-inner> '}';
block-inner := (<line>;)* <line>?;
line := (<expr>,)* <expr>?;

literal := <ident> | <number> | <string>
ident := ...;
number := ...;
string := ...;
