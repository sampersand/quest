Pristine.class = {
	(__args__.len() == 0).then(__args__.push << {});

	(__args__.len() == 2).then() {
		:1.__parents__ = [:1.__args__.shift()];
	};

	:0.instance_exec(__args__.pop());
	:0
};
