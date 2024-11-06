use std::fmt::Display;

/// A trait representing a Storey error.
///
/// This trait is implemented for all Storey error types, allowing third-party crates
/// to implement extension traits for all of those error types.
pub trait StoreyError: Display {}
