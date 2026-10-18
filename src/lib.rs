//! Crate root for the Rust practice repository.
//!
//! The exercises stay as standalone modules in `src/mainN.rs`. Each one is
//! compiled by its matching `tests/mainN.rs` integration test, which includes
//! the source directly so the private solution functions remain testable.
//!
//! This library target exists because the interactive `src/main.rs` runner and
//! the per-file `mainN()` demo functions have been removed. Cargo needs at
//! least one non-test target; run the suites with `cargo test`.
