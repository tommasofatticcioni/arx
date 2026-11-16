//! Arx command-line client.
//!
//! For now it only prints the version, which proves the workspace is wired
//! correctly: this binary crate compiles against the `arx-core` library crate.

// `fn main()` is the entry point, as in Java and C#. No class around it:
// Rust has free functions, and files are modules, not classes.
fn main() {
    println!("arx {}", arx_core::version());
}
