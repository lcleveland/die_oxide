//! Traits used by number generators.

/// A trait providing methods to generate and return an integer.
pub trait NumberGenerator {
    /// Generate and return an integer.
    fn generate(&self) -> i32;
}
