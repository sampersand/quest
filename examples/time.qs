# Hey look, time literals!
Time = object() {
	'()' = (cls, hours, seconds, ampm) -> {
		ampm = ampm.or('am');

		:0.becomes(Time)
	};

	@text = self -> {
		('' + self.hours) + ':' + (self.seconds) + ' ' + (self.ampm)
	};
};

# Hijack the `:` operator that's used for keyword arguments
Number.':' = Time;

print(12 : 40); # => 12:40 am

