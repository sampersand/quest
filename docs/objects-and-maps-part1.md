# Objects, Classes, and Maps (Part 1)
In most other languages, there's a clear distinction between classes, objects, and mappings. Every object is an instance of some class (which sometimes is an object itself)---with a few languages (such as Java) having a distinction between primitives and objects. Within these languages, a map is an instance of the `HashMap` class. In Quest, all three of these things are the same thing: executed `Block`s of code.

Among runtime languages, both classes and maps are able store data associated with some key: Classes only allow identifiers as "keys," storing them as instance variables. Mappings, on the other hand, allow for _anything_ to be used as a key, storing them in an internal table. Even though they both have different mechanisms for achieving this functionality, they both still _have_ it. 

## Representing Objects and Classes as Maps
Let's take a closer look at objects and classes. Generally, I could define classes and objects (in pseudocode) like so:
```
struct MyClass {
	parent_class := Class
	mixins := Array<Mixin> # also sometimes called "interfaces"
	constants := Map<String, Object>
	static_methods := Map<String, Function>
	instance_methods := Map<String, Function>
}

struct MyObject {
	object_id := int # defined by the VM, not modifiable by Quest programs.
	object_type := Class # (= MyClass)
	instance_variables := Map<String, Object>
}
```
Of the fields, there's two main categories: Those that are inheritance-related (ie `parent_class`, `interfaces`, and `object_type`) and those that aren't. It's simple to emulate the non-inheritance-related fields as a map: (again, pseudocode)
```
MyClass = {
	"constants": { "GREETING": "Hello, world." },
	# Constructors can be thought of as static methods for
	# classes that return an instance of the class.
	"static_methods": { "constructor": <function> },
	"instance_methods": { "greet": <function> }
}

MyObject = {
	"instance_variables": { "name": "Sam", "age": 22 }
}
```
The problem with this is that it completely omits any form of inheritance: How are you supposed to access the `greet` instance method from `MyClass` if `MyObject` is just what's defined above.

The solution that Quest uses is a special variable named `__parents__`: If you attempt to get an attribute of an object that doesn't exist, it walks through the `__parent__`s list, looking to see if _they_ have that attribute. Not only does this allow for inheritance, but it also gives us interface/mixin functionality for free! We can rewrite the previous example like such:
```
MyClassWithParents = {
	"__parents__": [<mixin1>, <mixin2>, ..., <parent class>],
	"constants": { "GREETING": "Hello, world." },
	"static_methods": { "constructor": <function> },
	"instance_methods": { "greet": <function> }
}

MyObjectWithParents = {
	"__parents__": [MyClassWithParents],
	"instance_variables": { "name": "Sam", "age": 22 }
}
```
Now, whenever you call `my_object.greet()`, it first checks to see if `MyObjectWithParents` "responds" to it (more on this later). If it doesn't (in this case, it'd only respond to `name`, and `age`), it traverses each parent in order, checking to see if that parent responds to it. If `my_obejct.__parents__` contains `MyClassWithParents`, we'll eventually call `MyClassWithParents`'s `greet` function.

## Method Resolution
A quick detour over to method resolution. The way I've laid out the examples above conforms to how many traditional programming languages divide up their classes. But you don't access constants via `MyClass.constants.GREETING` or get instance variables via `MyObject.instance_variables.name`---that'd be horrendous! Quest doesn't make the distinction between constants, instance variables, or even methods. Instead, they're all a part of every object's intrinsic map:
```
MyClassQuestVersion = {
	"__parents__": [<mixin1>, <mixin2>, ..., <parent class>],
	"GREETING": "Hello, world.",
	"()": <function>, # In quest, the constructor's a function named `()`
	"greet": <function>
}

MyObjectQuestVersion = {
	"__parents__": [MyClassQuestVersion],
	"name": "Sam",
	"age": 22
}
```
(Whenever you attempt to get a value, first every value within the object's checked, after which each parent within `__parents__` is checked.)

## Objects, Classes, and Mixins are the same concept
If you look at the example within the `Method Resolution` section, you'll notice that both `MyClassQuestVersion` and `MyObjectQuestVersion` are laid out in the exact same way: `__parents__`, and any other relevant information. This is not a coincidence, as objects and classes are identical in Quest! By abstracting away a lot of OOP concepts, we realize that the mechanism that is used to check for parent classes is the exact same one that's used to check for mixins/interfaces.

Because of this, there's _not a `Map` type in Quest_. (I know, I lied a little.) Since every object _already_ has key-value pairs associated with them, every object can be thought of as a map. (In practice, an empty `Scope` is usually used.)

# Final Remarks
Hopefully this post made it a little clearer why all these seemingly disparate concepts can all be unified by using the `__parents__` concept in concert with being able to add fields to any object. In [part 2](objects-and-maps-part2.md), we'll look at how these concepts can be put into practice.
