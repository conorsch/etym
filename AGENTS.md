# Agent Instructions for etym

The repository contains a Rust CLI tool called `etym` that queries
the website `https://etymonline.com` for words, and displays their etymologies.
It works by parsing the HTML from a fetch request.

## Build/Test Commands
- Build: `cargo build`
- Check: `cargo check`
- Test all: `cargo test`
- Test single: `cargo test <test_function_name>`
- Lint: `cargo clippy`
- Format check: `cargo fmt --check`
- Format: `cargo fmt`

## Code Style Guidelines

### Naming
- Functions: `snake_case`
- Structs/Enums: `PascalCase`
- Variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`

### Error Handling
- Use `anyhow::Result<T>` for fallible operations
- Use `?` operator for error propagation
- Use `anyhow::bail!` for early returns with custom errors
- Use `map_err` to convert error types

### Imports
- Group std imports first, then external crates
- Use explicit imports: `use crate::module::Item;`

### Formatting
- Use `cargo fmt` for consistent formatting
- Avoid uninlined format args (use `format!("{var}")` not `format!("{}", var)`)

### Documentation
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Include TODO comments for future work

### Dependencies
- anyhow: Error handling
- clap: CLI parsing
- regex: Pattern matching
- scraper: HTML parsing
- textwrap: Text formatting
- ureq: HTTP requests

### Testing
- Unit tests in `#[cfg(test)]` modules
- Use `include_str!` for test fixtures
- Integration tests in `tests/` directory
