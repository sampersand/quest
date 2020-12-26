Car = {
	@text = self -> { 
		"A" + ifl("aeiou".include?(self.maker[0]), 'n', '') +
			" " + self.maker + " with " + self.wheels +
			" wheel" + ifl(self.wheels == 1, '', 's')
	};

	drive = (self, distance) -> {

	};

	:0
}();
	Car = class(){
	  to_text = func(__self){
	    first_letter = __self.maker.0;
	    is_an = if(first_letter == 'a' || first_letter == 'e' ||
	               first_letter == 'i' || first_letter == 'o' ||
	               first_letter == 'u',
	               'n', '');
	    'a' + is_an + ' ' + __self.maker + ' with ' + __self.wheels + ' wheels'
	  };
	  drive = func(__self, distance){
	    disp("I drive " + __self.to_text(), end = '. ';);
	    DIDNT_DRIVE = 'Haha, I didn\'t drive.';
	    DID_DRIVE = "I drove " + distance + " mile" + (distance == 1 ? "" : "s")!.0 + ".";
	    disp(distance == 0 ? DIDNT_DRIVE : DID_DRIVE);
	  }
	};

	car = Car(wheels = 4, maker = 'Honda');
	car.drive(0);
	car.drive(1);
	car.drive(2);

