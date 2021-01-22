TimeExt = {
  milliseconds = self -> { self };
  seconds = self -> { 1000 * self.milliseconds() };
  minutes = self -> { 60 * self.seconds() };
  hours = self -> { 60 * self.minutes() };
  days = self -> { 24 * self.hours() };
  weeks = self -> { 7 * self.days() };
  months = self -> { 30 * self.days() };
  years = self -> { 365 * self.days() };

  :0
}();

# Either add it to individual numbers:
print(12.extend(TimeExt).weeks());
print(34.extend(TimeExt).seconds());

# Or add it to `Number` directly:
Number.extend(TimeExt);
print(12.weeks());
print(34.seconds());
