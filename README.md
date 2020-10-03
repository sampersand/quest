# Quest

[![CI](https://github.com/sampersand/quest/workflows/CI/badge.svg)](https://github.com/sampersand/quest/actions)

A language based around extensibility and freedom

# What's Quest
Quest is an "non-typed" language that is designed to allow for efficient code reuse. Similar to dynamically-typed languages in that types aren't relevant, Quest takes this a step further: There _are_ no types (just key-value pairs).

# Features
Quest supports everything you'd expect from a programming language and more!
- No "keywords"
- No syntactic distinction between classes, functions, and maps/dicts/hashmaps.
- No distinction between [l- and r-values](https://en.wikipedia.org/wiki/Value_%28computer_scienc%29#lrvalue)
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
```quest
# Text can either be single or double quotes: they're identical (like python).
$where = "world";
disp("Hello, " + where + "!"); # => Hello, world!
```

Local variables are actually just string keys on the current object. The following is identical to the previous example:
```quest
__this__."where" = "world";
disp("Hello, " + __this__."where" + "!"); # => Hello, world!
```

## Functions, Classes, and Maps

In Quest, there are no named/anonymous functions—they're both simply `Block`s, written as `{ ... }`:
```quest
# Arguments are passed via local variables `_1`, `_2`, etc.
# The last statement in a block is implicitly returned.
disp("4 squared is:", { _1 ** 2 }(4)); # => 4 squared is: 16

# You can assign anonymous functions to variables too:
$square = { _1 ** 2 };
disp("4 squared is:", square(4)); # => 4 squared is: 16

# You can even just straight-up add them to builtin classes:
# The `_0` argument is the the object that this method was called on, akin to
# `self` or `this` in other languages.
Number.$square = { _0 ** 2 };
disp("4 squared is:", 4.$square());
```

Maps are created by simply returning the result of an executed block of code:
```quest
$traffic_lights = {
	# A blank scope is created whenever a block is called.

	__this__."red" = 'stop';
	__this__."green" = "go";
	__this__."yellow" = "go, if you can";

	__this__ # Return the current scope
}();

disp("Green means", traffic_lights."green"); # => Green means go
```

**Classes are actually just objects too**: They're just a group of methods which are available for any object which makes the "class" its parent. There is no intrinsic concept of a "constructor" either, and is generally implemented by overloading the "call" (`()`) function and returning the new scope.
```quest
$Person = {
	# `$xxx` is identical to `"xxx"`. So the following could be written as `"()" = { ... };`

	# Whenever something is called, the `()` attribute is run.
	# "Constructors" are really just defining a `()` attribute that overwrites `__parents__` and
	# returns `__this__` as the last value.
	$() = {
		# You can have multiple parents (to allow for multiple inheritance and mixins).
		# However, here we don't need to have multiple parents.
		$__parents__ = [Person];

		$first = _1;
		$last = _2;

		__this__ # return the current scope, i.e. the new object
	};

	# Define the conversion to a text object
	$@text = {
		# the 0th argument is the object this method was called upon; it
		# generally equates to the `self`/`this` of other languages.
		_0.$first + " " + _0.$last
	};

	__this__ # like in `traffic_lights`, we return the current scope.
}();

$person = Person("John", "Doe");
disp(person); # => "John Doe"
```

## No Keywords
Sticking to the theme of extensibility and freedom, there aren't traditional "keywords." Traditional control-flow keywords (such as `if`, `while`, and `return`) are simply attributes defined on the `Kernel` object (which most objects inherit from). And traditional "definition" keywords (such as `class` and function-declaration keywords) aren't relevant.

```quest
$factorial = {
	# The if function executes whichever branch is chosen
	if(_1 <= 1, {
		1
	}, {
		_1 * factorial(_1 - 1)
	})
};
disp("10! =", factorial(10)); # => 10! = 3628800

$i = 0;
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


```quest
$make_dinner = {
	$the_magic_word = prompt("what's the magic word? ");
	if(the_magic_word != "please", {
		disp("You didn't say 'please'!");
		# `:0` is the current stackframe, `:1` is the stackframe above this one
		# in this case, that's the `$make_dinner` stackframe. return `false` from
		# that stackframe.
		return(:1, false);
	});

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
```quest
$i = 0;
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
});}

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
```quest
# remember `$xxx` is identical to `'xxx'`
$x = 5; # call the `Text::=` function implicitly
$y.$=(6); # call the `Text::=` function explicitly

disp(__this__.$x, __this__.$y); # => 5 6

Number.$= = Text::$=; # now you can assign numbers.

3 = 4;
disp(__this__.3) # => 4
```

(Minor note: `a.b = c` doesn't actually use the `=` operator; it's syntactic sugar for the `.=` operator—`a.$.=(b,c)`—and is accessible on every object that inherits from `Pristine` (which is everything, by default).)

`Text` also has the `()` method defined, where it simply looks up its value in the current scope: (Bare variables, eg `foo`, were added so `$foo()` wouldn't be necessary.)

```quest
$x = 5;
disp(x, $x(), 'x'(), __this__.'x', __this__.$x); # => 5 5 5 5 5
```

## Everything is fair game
Most runtime languages support some form of instance variables that can be added to objects. However, Quest takes this a step further, and allows _everything_ to have attributes added/removed from them, including primitives like numbers. (For those language-savvy folks, every Quest object is a singleton object.)

```quest
# define the `square` method on 
Number.$square = { _0 ** 2 };

$twelve = 12;
disp(twelve.$square()); # => 144

# define the `cube` method on this instance of 12.
twelve.$cube = { _0 ** 3 };
disp(twelve.$cube); # => 1728

# no other `12` in the program has access to the `cube` method.
disp(12.$__has_attr__($cube)); # => false
```

## More
See the `examples` folder for more examples of what Quest can do!

There's also some stuff I've written up in the `docs` that goes more into depth.

## TODO
I should probably add more discussion of Quest's features.



# Misc
## Formal syntax Specification
```

code := (expr;)* expr
expr := primary | expr <binary-op> expr | function_call
primary := <unary-op> expr | block | <literal>
function_call := expr block
block := '(' block_inner ')' | '[' block_inner ']' | '{' block_inner '}'
block_inner := (line;)* (line)?
line := (expr,)* (expr)?
```






