## Example 1: Time
TimeExt = object() {
  seconds = self -> { self };
  minutes = self -> { 60 * self.seconds() };
  hours = self -> { 60 * self.minutes() };
  days = self -> { 24 * self.hours() };
  weeks = self -> { 7 * self.days() };
  months = self -> { 30 * self.days() };
  years = self -> { 365 * self.days() };
};

# Either add it to individual numbers:
print(12.extend(TimeExt).weeks());
print(34.extend(TimeExt).hours());

# Or add it to `Number` directly:
Number.extend(TimeExt);
print(12.weeks());
print(34.hours());

quit(); # example two doesn't work yet, as `Io::Dir` doesn't exist

## Example 2: Paths
Path = object() {
  file?   = self -> { Io::File.exist?(self) };
  dir?    = self -> { Io::Dir.exist?(self) };
  open    = (self, mode='w') -> { Io::File(self, mode) };
  touch   = self -> { Io::File(self).write(""); };
  mkdir_p = self -> { Io::Dir.mkdir_p(self) };
  parent  = self -> { self.sub('/[^/]*$'.@regex(), '').extend(Path) }; 
};

path = "/tmp/some/random/path/log.txt".extend(Path);

unless (path.parent().dir?()) {
	path.parent().mkdir_p();
};

path.parent().dir?().else(path.parent().mkdir_p);

path.open().write("hello!");
