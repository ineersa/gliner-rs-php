### Build and configuration
- This is a Rust workspace with a single binary crate (`gliner-rs-php`). Build with `cargo build` from the project root.
- The crate targets Rust `edition = "2024"` (see `Cargo.toml`). Ensure your toolchain supports the 2024 edition.

### Testing
- Run the full test suite from the project root:
  ```
  cargo test
  ```
- Add new unit tests in `src/main.rs` under the `#[cfg(test)]` module, or create additional modules under `src/` and add `#[cfg(test)]` blocks there.

#### Verified example (run before documenting)
The following test is already present in `src/main.rs` and was executed successfully:
```
cargo test
```

### Development notes
- The project currently has a minimal binary entrypoint in `src/main.rs`. Keep new helpers small and testable; place pure functions above `main` and add unit tests in the same file.
- Follow standard Rust formatting (`rustfmt`) and existing style conventions in `src/main.rs` (4-space indentation, snake_case names).