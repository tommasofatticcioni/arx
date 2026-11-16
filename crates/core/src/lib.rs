//! Arx core library.
//!
//! Everything that touches secrets lives in this crate: cryptography, the key
//! hierarchy, and the vault data model. The CLI, the server client, the app
//! and the extension all call into it.
//!
//! (Lines starting with `//!` are documentation for the enclosing item, here
//! the whole crate. Lines starting with `///` document the item that follows.
//! `cargo doc --open` renders both as HTML, like Javadoc or XML doc comments.)

/// Returns the version of the core library, for example `"0.1.0"`.
///
/// Reading this signature:
/// - `pub` exports the function from the crate. Without it the function is
///   private to this module. Private is the default in Rust, as with a
///   TypeScript declaration that has no `export`.
/// - `-> &'static str` is the return type. `&str` is a borrowed, read-only
///   view of UTF-8 text owned by someone else. `'static` is a *lifetime*: it
///   says the text lives for the entire run of the program, which is true for
///   text baked into the executable. Lifetimes are the big new idea in Rust
///   and we will go into them properly in milestone 1; for now read
///   `&'static str` as "a string constant".
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// Unit tests live in the same file as the code they test, in a child module.
// `#[cfg(test)]` is conditional compilation: this module only exists when
// running `cargo test`, so tests never end up in the real build.
#[cfg(test)]
mod tests {
    // A child module does not see its parent's items automatically.
    // `super` is the parent module; `*` imports everything from it.
    use super::*;

    // `#[test]` marks a test function, like @Test in JUnit or [Fact] in xUnit.
    #[test]
    fn version_is_not_empty() {
        // Call the function under test, not `env!` directly: the test should
        // fail if `version()` is ever changed to return something else.
        // `!` negates: we assert the string is NOT empty.
        assert!(!version().is_empty());
    }

    #[test]
    fn version_matches_the_package_version() {
        // `assert_eq!(left, right)` fails and prints both values when they
        // differ, like assertEquals / Assert.Equal.
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }
}
