use quest_runtime::{QuestVm, ByteCode, RegisterIndex};


fn main() {
	use ByteCode::*;
	use quest_core::Literal;
	quest_core::init();

	const REG0: RegisterIndex = RegisterIndex::Scratch(0);
	const REG1: RegisterIndex = RegisterIndex::Scratch(1);
	const REG2: RegisterIndex = RegisterIndex::Scratch(2);

	let vm = QuestVm::new(vec![
		StoreI(REG0, 10),
		StoreI(REG1, 1),
		LoadL(REG2, Literal::new("disp")),
		Push(REG0),
		CallAttrL(REG2, Literal::new("()"), 1),
		Sub(REG0, REG1),
		
		Cmp(REG0),
		Jne(-5),
		Nop,
	]);

	vm.print_program();

	vm.run();
}
