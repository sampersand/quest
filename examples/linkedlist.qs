Node = object() {
	'()' = (class, data, next) -> { :0.becomes(class) };
	@text = node -> { node.data.@text() };
};

LinkedList = object() {
	'()' = (class) -> {
		head = null;
		:0.becomes(class)
	};

	push = (self, value) -> {
		self.head = Node(value, self.head);
	};

	pop = (self) -> {
		self.head.else(return).tap({ self.head = self.head.next })
	};

	@text = self -> { self.@list().@text() };

	@list = self -> {
		node = self.head.else(return);
		acc = [node.data];

		while({ :1.node = node.next }, {
			acc.push(node.data);
		});

		acc 
	};
};

l = LinkedList();
l.push(3);
l.push(4);
print(l);
print(l.pop());
print(l.pop());
