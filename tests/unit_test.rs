//! Project-level checks for the per-file test suite.
//!
//! Every `src/mainN.rs` exercise must have a matching `tests/mainN.rs`.
//! The individual suites include the source file as a private module, which
//! lets them exercise functions that are not `pub`.

use std::path::Path;

#[test]
fn every_exercise_has_a_matching_test_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = root.join("src");
    let tests_dir = root.join("tests");
    let mut checked = 0;

    for entry in std::fs::read_dir(&src_dir).expect("src directory should be readable") {
        let entry = entry.expect("src entry should be readable");
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        let is_exercise =
            name.starts_with("main") && name.ends_with(".rs") && name.as_ref() != "main.rs";
        if !is_exercise {
            continue;
        }

        let test_file = tests_dir.join(name.as_ref());
        assert!(test_file.is_file(), "missing test suite: tests/{name}");
        checked += 1;
    }

    assert_eq!(
        72, checked,
        "expected one test suite for each of the 72 exercises"
    );
}
