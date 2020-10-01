
# What this means in practice
It's all fine-and-dandy to talk about how all these concepts are the same _theoretically_, but it's important to know how to put this knowledge to use. Syntactically, Quest emulates all these concepts (ie classes/objects/mixins/maps) via the syntax `$ThignyName = { ...; __this__ }();`. (In a later post we'll take a closer look at why this syntax was chosen, but for now roll with it :P). With that in mind, let's take a look at the different concepts and how we can emulate them.

## Emulating Classes and Objects
To start off with, we look at how you would emulate classes and their instances within Quest. Let's make an `Animal` "class" and instantiate it:
```quest
$Animal = {
	$() = {}
}


# Why is this Helpful
This pseudo-prototype-based inheritance allows for a _lot_ of freedom: Being able to add any object to `__parents__` means you're not constrained by the traditional "class -> instance" inheritance mechanism. Take a look:

## Emulating Classes
We'll first look at how you emulate classes. 

## Emulating mixins.
Syntactically, Quest emulates classes/mixins/maps via `{ ...; __this__ }()`. (In a later post, we'll look more at how maps and classes are similar.) Let's make a `Greeter` mixin, which simply adds a `greet` method:
```quest
# Assign `Greeter` to the anonymous mixin we make:
$Greeter = {

	# Assign `greet` to the anonymous function:
	$greet = {

		# `_0` is analogous to `this`/`self` in other languages.
		disp("Hello,", _0);
	};

	__this__
}();
```
Simple enough.

# You can add parents to specific objects directly:
$sam = "Sam";
sam.$__parents__.$push(Greeter);
sam.$greet(); # => Hello, Sam

# No other instance of `"Sam"` will have this defined:
"Sam".$greet(); # method not found


# You can also add it to "classes" too:
Number.$__parents__.$push(Greeter);
12.$greet(); # => Hello, 12

# Heck, you can add it to the current scope:
__parents__.$push(Greeter);
greet(); # => Hello, <main>
```

## Changing/Removing `__parents__`
