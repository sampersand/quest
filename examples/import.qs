# I currently haven't implemented a true "import" functionality et
Io.File("./frac.qs").read().eval();
half = Frac(1, 2);

print(half); # => 1/2
print(half + 2); # => 5/2
print(half < 0.75) # => true


# Tests
assert(half.@text() == "1/2");
assert((half + 2).@text() == "5/2");
assert(half < 0.75);
