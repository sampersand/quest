# I currently haven't implemented a "read file" functionality yet...
Io.File("./frac.qs").read().eval();
half = Frac(1, 2);

print(half); # => 1/2
print(half + 2); # => 5/2
print(half < 0.75) # => true


# Tests
assert(half.@text() == "1/2");
assert((half + 2).@text() == "5/2");
assert(half < 0.75);
