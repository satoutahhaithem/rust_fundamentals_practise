# Copilot Instructions for Rust Fundamentals Repository

## Project Overview

This is a **Rust learning repository** from a 4-week bootcamp (Week 2-4 content). It's structured as a workspace with multiple independent mini-projects demonstrating Rust concepts, NOT a single production application. Each subdirectory is a standalone example or lab exercise.

## Repository Structure

- **Root workspace**: Defines `edition = "2024"` in Cargo.toml (note the edition year convention)
- **Week-based organization**:
  - `week_2_rust_fundamentals/` - Borrowing, functions, loops, error handling, panic
  - `week_3_structs_types_and_enums/` - Structs, enums, vectors, strings
  - `week_4_apply_rust/` - Libraries, documentation, debugging patterns
- **Each week contains multiple independent crates** (not a monorepo with shared deps)
- Many projects have their own `target/` directories - this is intentional for isolated learning

## Key Patterns & Conventions

### Cargo Workspace Usage
- Use `-p <package>` flag for building specific projects: `cargo build -p sample_app`
- Tasks are configured for workspace builds (see `.vscode/tasks.json`)
- Example: `sample_app` depends on local library via `my_library = { path = "../my_library" }`

### Error Handling Philosophy
- **Explicit `match` on `Result` and `Option`** is the teaching pattern (see [lab_error_handling_with_match/src/main.rs](week_2_rust_fundamentals/lab_error_handling_with_match/src/main.rs))
- Match on `std::io::ErrorKind` variants for granular error handling
- Uses `panic!` for unrecoverable errors with descriptive messages
- Avoid `unwrap()` in examples; prefer explicit error handling to demonstrate concepts

### Ownership & Borrowing Patterns
- Functions demonstrate ownership transfer vs borrowing explicitly
- Example pattern from [borrowing_in_rust/src/main.rs](week_2_rust_fundamentals/borrowing_in_rust/src/main.rs):
  - `own_integer(i32)` - takes ownership (Copy trait makes it work)
  - `own_string(String)` - moves ownership (invalidates original)
  - `own_vec(&Vec<i32>)` - borrows immutably (original remains valid)
- Comments explain ownership semantics inline

### Documentation Standards (Week 4)
- Module-level docs use `//!` (inner doc comments) at file top
- Function docs use `///` with `# Examples` sections containing doctests
- See [week_4_apply_rust/doc_your_code/src/colors.rs](week_4_apply_rust/doc_your_code/src/colors.rs) for reference
- Re-export pattern: `lib.rs` declares `pub mod` for submodules

### Struct & Enum Conventions
- Structs use `#[derive(Debug)]` for printability in examples
- Implement methods via `impl` blocks with `&self` for borrowing semantics
- Enums often used for type-safe variants (see `FileSize` enum in [week_3_structs_types_and_enums/lab_enum/src/main.rs](week_3_structs_types_and_enums/lab_enum/src/main.rs))
- Pattern matching on enums demonstrates Rust's exhaustiveness checking

### Testing
- Tests live in `#[cfg(test)] mod tests` within source files
- Use `assert_eq!` for value assertions
- Example: [week_4_apply_rust/my_library/src/lib.rs](week_4_apply_rust/my_library/src/lib.rs) contains comprehensive unit tests
- Root `src/lib.rs` includes basic test scaffolding

## Development Workflows

### Building Projects
```bash
# Build entire workspace
cargo build

# Build specific package (preferred for isolated work)
cargo build -p sample_app
cargo build -p debug_

# Use Makefile shortcuts (root level)
make build  # builds workspace
make lint   # runs clippy
make fmt    # formats code
```

### Running Examples
Each week's projects are standalone binaries. Navigate to the project directory or use:
```bash
cargo run -p <package_name>
```

### Documentation Generation
For the `doc_your_code` project:
```bash
cd week_4_apply_rust/doc_your_code
cargo doc --open  # generates and opens HTML documentation
```

## Common Pitfalls for AI Agents

1. **Edition is "2024"** - not a typo. Some projects may require adjusting if using older Rust features.
2. **Don't consolidate projects** - each subdirectory is intentionally isolated for pedagogical purposes.
3. **Terminal output examples** - Projects like [debug_/src/main.rs](week_4_apply_rust/debug_/src/main.rs) are designed for interactive terminal I/O (e.g., reading stdin until "stop").
4. **Path dependencies** - When editing projects with dependencies like `sample_app`, remember the relative path structure: `{ path = "../my_library" }`.
5. **Match exhaustiveness** - When modifying enums, ensure all match arms are covered or use `_` wildcard.

## Project-Specific Notes

- **sample_app**: Demonstrates library consumption pattern with free functions, struct methods, and enum operations
- **my_library**: Export pattern showing both public functions and public types (Calculator struct + Operation enum)
- **doc_your_code**: Focus on rustdoc syntax - module docs (`//!`), function docs (`///`), and examples
- **lab_enum**: Shows multiple formatting approaches (enum-based vs. procedural) for the same problem

## CI/CD
- GitHub Actions workflow (`.github/workflows/main.yml`) runs `cargo build --verbose` on PRs to main
- Configured for Ubuntu runner with stable Rust
- No test execution in CI currently (tests are in-file only)
