## Cargo Commands

This project uses standard Cargo commands for development and compilation:

* **`cargo check`**: Quickly checks your code to make sure it compiles without producing an executable binary. Great for finding errors fast while developing.
* **`cargo build`**: Compiles the project and produces an unoptimized debug binary (located in `target/debug/`).
* **`cargo run`**: Compiles and executes the project in a single step.
* **`cargo build --release`**: Compiles the project with optimizations for production. This takes longer to compile, but the resulting binary runs much faster (located in `target/release/`).
