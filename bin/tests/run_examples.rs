use std::fs::read_dir;
use std::path::Path;
use std::process::Command;

// Examples to skip. Right now, just `guessing_game` which waits for user input.
//
// Ideally we'd separate tests from examples or we'd have a way of indicating
// this, but for a single item it's not worth it.
const SKIP: &[&str] = &["guessing-game.qs", "knight/guess.kn"];

fn should_skip(p: &Path) -> bool {
	SKIP.iter().any(|to_skip| p.ends_with(*to_skip))
}

#[test]
fn run_examples() {
	let exe = dbg!(env!("CARGO_BIN_EXE_quest-bin"));
	let manifest_dir = dbg!(env!("CARGO_MANIFEST_DIR"));
	let examples_dir = dbg!(Path::new(manifest_dir).join("../examples"));
	let mut failed = false;
	for example in read_dir(examples_dir).unwrap() {
		let example = example.unwrap();
		eprintln!("Running example {:?}... ", example);
		let example_path = example.path();
		if should_skip(&example_path) {
			eprintln!("\tSKIP");
			continue;
		}

		let mut cmd = Command::new(exe);
		// Todo: Should run with timeout
		let out = cmd
			.arg("-f")
			.arg(&example_path)
			.output()
			.unwrap_or_else(|e| {
				panic!(
					"Failed to run: `{} -f {}`: {:?}",
					exe,
					example_path.display(),
					e,
				)
			});
		if out.status.success() {
			eprintln!("\tPASS");
		} else {
			eprintln!("\tFAIL\nOutput: {:#?}", out);
			failed = true
		}
	}
	assert!(
		!failed,
		"One or more tests failed to run, see stderr for details"
	);
}
