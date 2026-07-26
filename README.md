## Cargo Commands

This project uses standard Cargo commands for development and compilation:

* **`RUSTFLAGS="-A dead_code -A unused" cargo check`**: Quickly checks your code to make sure it compiles without producing an executable binary. Great for finding errors fast while developing.
* **`RUSTFLAGS="-A dead_code -A unused" cargo build`**: Compiles the project and produces an unoptimized debug binary (located in `target/debug/`).
* **`RUSTFLAGS="-A dead_code -A unused" cargo run`**: Compiles and executes the project in a single step.
* **`RUSTFLAGS="-A dead_code -A unused" cargo build --release`**: Compiles the project with optimizations for production. This takes longer to compile, but the resulting binary runs much faster (located in `target/release/`).
