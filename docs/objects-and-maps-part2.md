# Objects, Classes, and Maps (Part 2)
In the [part 1](../objects-and-maps-part1.md), we explored how objects, classes, mixins (and interfaces), and maps were all fundamentally able to be represented by the same construct within Quest (with the help of `__parents__`).


## A Quick tl;dr of Part 1
Classes, objects, and mixins all fundamentally share two concepts---mappings and inheritance. They all have some mapping between identifiers to "values" (e.g. constants, static functions, methods, or instance variables) of some sort. Additionally, they all include some form of indirection: If the current object doesn't directly know how to respond to a method call, you go up the inheritance chain until you find something that can.

Quest allows every object to have arbitrary key-value pairs declared on them---this implements the first concept by allowing you to define constants, instance variables, etc., on any object you want. Inheritance is implemented by a special key `__parents__`, a list of objects that will be iterated over until the unknown key is found.

## A Quick Note on Syntax
Because all these ideas are conceptually the same thing within Quest, there's no special syntax to distinguish them from one another. Instead, you make "anonymous `Scope`s," and assign them to identifiers with the syntax `$MyThingy = { ...; :0 }();`. In part 3, we talk more about this and why we use it.

## Emulating Maps in Quest
Let's put some of this theory into practice and look at how maps are created within Quest:
```quest
# Assign the map to the variable named `stoplights`.
$stoplights = {
	$red = "stop!";
	$green = "go!";
	$yellow = "go! (carefully)";

	:0
}();

disp("Red means", stoplights.$red); # => "Red means stop!"
disp("Green means", stoplights.$green); # => "Green means go!"
```
Fairly simple. Where it gets interesting is when we add in `__parents__`.

## Emulating Classes in Quest
Our fist excursion into using `__parents__` in practice comes in the form of classes. To begin with, let's make an extremely simple `Person` class:
```quest
# Use the "map" syntax (`{ ...; :0 }();`) to create a "class".
$Person = {
	# Assign a "constant" to the `Person` "class".
	$SAYS_WHAT = "Hello!";

	# Define the conversion to `Text` for `Person`s. In Quest, conversion
	# functions are no different than any other function. They typically start
	# with the `@` symbol, such as `@text` or `@num`.
	$@text = {
		# `_0` is analogous to `this`/`self` in other languages.
		_0.$first + ' ' + _0.$last
	};

	# Have the person speak.
	$speak = {
		# Arguments passed to `disp` are automatically converted to `Text` (via
		# `@text`) and have spaces separating them.
		disp(_0, "says:", _0.$SAYS_WHAT);
	};

	:0
}();
```
Here we've made a Person "class" with a constant and two associated methods. Let's create an instance of it:
```quest
# Again, we use the "map" syntax (`{ ...; :0 }();`) to create an "object".
$sam = {
	# Override the default parents with what we want the superclass to be: Person.
	$__parents__ = [Person];

	# Assign associated values.
	$first = "Sam";
	$last = "W";

	:0
}();

# Let's have him speak.
sam.$speak(); # => "Sam W says: Hello!"
```

This, of course, is not ideal. We don't want to have to manually assign `__parents__` _every single time_ we make a new object. Instead, let's add a function to the `Person` class that will do it for us; this is analogous to constructors in other languages, except in Quest there's nothing special about it. Typical Quest code overloads the "call" operator (`()`) when writing constructors, allowing you to say `$my_obj = MyClass(arg1, arg2, ...);`.
```quest
# (Normally, we would have added this to the definition of `Person` when we
# created it earlier. However, since it's already created we may as well just
# add the function onto it manually.)
Person.$() = {
	# `_0` will be whatever class this method was called on. See part 3 for more
	# details on this. (For this example, it'll be `Parent`.)
	$__parents__ = [_0];

	# `_1` and `_2` are the first and second arguments, respectively, to the call
	# operator.
	$first = _1;
	$last = _2

	:0
}; # Note the lack of the `()` at the end. We talk about this in part 3.


# Now we create a person
$sam = Person("Sam", "W");
sam.$speak(); # => Sam W says: Hello!
```
We now have a basic example of a class in Quest! Huzzah.

### Adding inheritance to Classes.
That's all fine and dandy, but it's not very useful. What if we wanted to add in a child class, say a `Baby` class? We'll want to leverage the `__parents__` feature to make the `Baby` a subclass of `Person`. However, this time, instead of adding the parents to each _instance_ of a `Baby`, we add it to the `Baby` itself: That way, anything that has `Baby` as a parent will _also_ have `Person` as a "grandparent".
```quest
$Child = {
	# This is analogous to saying "`Child` is a subclass of`Person`."
	$__parents__ = [Person];

	# Babies want food, not to greet people
	$SAYS_WHAT = "Waa! I want food!";

	# The prefix to be used in `@text`.
	$BABY_PREFIX = "Baby"

	# Let's also override the `@text` representation so the `speaks` method has
	# a correct representation.
	$@text = {
		_0.$BABY_PREFIX + " " + _0.$first
	};

	:0
}();

# We can then make a baby like so:
$baby_sam = Baby("Sammy", "W");
baby_sam.$speak(); # => Baby Sammy says: Waa! I want food!
```
With that, we now have inheritance working via clever use of `__parents__`! For completeness, here is what `baby_sam`'s attributes list would look like in yaml:
```yaml
# `baby_sam`
first: "Sammy"
last: "W"
__parents__:
  # `Child`
  - SAYS_WHAT: "Waa! I want food!"
    BABY_PREFIX: "Baby"
    @text: { "Baby " + _0.$first }
    __parents__:
      # `Person`
      - SAYS_WHAT: "Hello!"
        @text: { _0.$first + ' ' + _0.$last }
        speak: { disp(_0, "says:", _0.$SAYS_WHAT); }
        (): |
          {
            $__parents__ = [_0];
            $first = _1;
            $last = _2;
            :0
          }
        __parents__:
          # Irrelevant for this example
```

## Mixins
Lastly, we get to mixins. They're not too conceptually different from the other things we've discussed so far, as they're just something else we add into the `__parents__` hierarchy. Quest has a few mixins built in, such as `Comparable` (which gives you the functions `<`, `<=`, `>`, and `>=` in terms of `<=>`) and `Iterable` (which provides methods like `map`/`select` in terms of `each`), but we'll be building our own for this example:
```quest
# Once again, we use the `{ ...; :0 }()` syntax, this time for a mixin.
$Greeter = {
	$greet = {
		# `_0` is analogous to `self`/`this` here, and will be converted to a Text
		# before being printed out.
		disp("Hello,", _0);
	};
	:0
}();
```
As you can see, it's structured in the same was as the classes we used earlier. Adding it to objects is quite simple too:
```quest
$location = "world";
# We want to add `Greeter` to the parents, not overwrite them with _only_
# Greeter.
location.$__parents__.$push(Greeter);
location.$greet(); # => Hello, world

# Note though that this will only update `location`'s `__parents__`:
"world".$greet(); # error, 'greet' not found!

# You can add mixins to anything, including classes like Number:
Number.$__parents__.$push(Greeter);
93.$greet(); # => Hello, 93
```
As you can see, mixins can be used in the exact same way classes are: You simply add them to the list of parents, and they'll automatically become part of the hierarchy

# Final Remarks
Whew, that article was much longer than I intended it to be. [Part 3](../objects-and-maps-part3) discusses the `{ ...; :0 }()` syntax, explaining why _functions_ and objects can be constructed with the same syntax!
