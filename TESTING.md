# Running the tests

Every exercise has a dedicated integration-test file:

```
src/main1.rs  <->  tests/main1.rs
src/main2.rs  <->  tests/main2.rs
...
src/main72.rs <->  tests/main72.rs
```

Each test file includes its exercise as a private module:

```rust
mod main1 {
    #![allow(dead_code, unused_imports, unused_mut)]
    include!("../src/main1.rs");

    #[cfg(test)]
    mod tests {
        use super::two_sum;
        // ...
    }
}
```

`include!` is used so the tests can call private helper functions without
changing the practice source files. Each test file is a separate Cargo
integration-test target, which is what makes it independently runnable from an
IDE.

`tests/unit_test.rs` is a project-level check that every `src/mainN.rs` has a
matching `tests/mainN.rs`.

The interactive `src/main.rs` runner and each file's `mainN()` demo function
have been removed. Tests are now the only entry point, and `Cargo.toml` builds
a small `src/lib.rs` target so Cargo can discover and run every integration
test without a binary target.

## Cargo setup

`Cargo.toml` now contains a library target and no binary target:

```toml
[lib]
name = "rust_practice"
path = "src/lib.rs"
```

Cargo auto-discovers all `tests/*.rs` files, so no per-exercise `[[test]]`
entries are needed. `cargo run` is intentionally unavailable now; use
`cargo test` instead.

## Command line

```bash
# Run everything
cargo test

# Run all tests for one exercise
cargo test --test main12

# Run one test by name
cargo test --test main12 top_is_past_the_last_step

# Run the documented known-issue tests (they are expected to fail)
cargo test -- --ignored
```

## IntelliJ IDEA / CLion / RustRover

1. Open the project by selecting the project root (the directory containing
   `Cargo.toml`).
2. Wait for the Cargo project to sync and indexing to finish.
3. Open any `tests/mainN.rs` file. Green run arrows appear in the gutter next
   to each `#[test]` function and next to each module.
4. Click the gutter arrow to run/debug a single test, or right-click
   `tests/mainN.rs` and choose **Run 'mainN'** to run the whole suite for that
   exercise.
5. To run everything, right-click the `tests` directory and choose
   **Run 'All Tests'**, or create a Cargo run configuration with:
   - **Command:** `test`
   - **Working directory:** the project root

The test tree is also available through **View | Tool Windows | Cargo**.
If gutter icons do not appear, check that the Rust plugin is enabled and that
the Cargo project has synced. The tests are ordinary Cargo integration tests,
so `cargo test` always works even without IDE integration.

## VS Code

Install the `rust-analyzer` extension, open the project root, and use the Test
Explorer or the **Run Test** / **Debug Test** CodeLens links above each
`#[test]`.

## Known implementation issues

The suite is green: **399 tests pass and 15 are intentionally ignored**. The
ignored tests assert the expected LeetCode behaviour and document bugs in the
current implementations. They are kept in the suite so they can be enabled
again after a fix.

| File | Ignored tests | Issue |
| --- | --- | --- |
| `main6` | 1 | `can_be_increasing_fast` counts rule breakers instead of trying each removal |
| `main19` | 2 | `generate_parenthesis` drops branches; n=2 returns only `(())` |
| `main47` | 1 | `delete_and_earn` memo key omits selection state; `[2,2,3,3,3,4]` returns 8 instead of 9 |
| `main51` | 3 | Zigzag conversion repeats outer-row skips for middle rows; one row emits only the first character |
| `main53` | 1 | `find_duplicate` compares values against zero-based indices |
| `main61` | 3 | DFS adds the terminal node's score twice; examples return 30/13 instead of 24/12 |
| `main65` | 2 | `four_sum` is an unimplemented stub that always returns an empty vector |
| `main69` | 2 | A trailing `*` cannot match an empty remainder because the base case rejects it |

Run a specific ignored case to see the actual failure, for example:

```bash
cargo test --test main47 -- --ignored
```
