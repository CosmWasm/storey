//! A set of traits for encoding and decoding data.
//!
//! # Overview
//!
//! The [`Encoding`] trait is for types that serve as "markers" for a particular encoding.
//!
//! The [`EncodableWith`] and [`DecodableWith`] traits are for types that can be encoded
//! and decoded with a particular encoding, respectively.
//!
//! [`NonNullItem`]: crate::containers::NonNullItem
//!
//! # Implementing an encoding
//!
//! To implement an encoding, you need to provide a type that implements [`Encoding`].
//! These types are generally zero-sized unit structs.
//!
//! You must also provide blanket implementations for encodable/decodable types.
//!
//! The [`EncodableWith`] and [`DecodableWith`] traits are [sealed], so you can't implement
//! them directly. Instead, what you want to do is implement [`EncodableWithImpl`] and
//! [`DecodableWithImpl`], using [`Cover<&T>`] and [`Cover<T>`] to [cover] the type
//! parameters.
//!
//! [cover]: https://doc.rust-lang.org/reference/glossary.html#uncovered-type
//! [sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
//!
//! ## Encoding example
//!
//! ```
//! use storey::encoding::{EncodableWithImpl, Encoding, Cover};
//!
//! // - Implementation -
//!
//! struct DisplayEncoding;
//!
//! impl Encoding for DisplayEncoding {
//!     type DecodeError = String;
//!     type EncodeError = String;
//! }
//!
//! impl<T> EncodableWithImpl<DisplayEncoding> for Cover<&T,>
//! where
//!     T: std::fmt::Display,
//! {
//!     fn encode_impl(self) -> Result<Vec<u8>, String> {
//!         Ok(format!("{}", self.0).into_bytes())
//!     }
//! }
//!
//! // - Usage -
//!
//! use storey::encoding::EncodableWith as _;
//!
//! // If there's only one encoding present for `u64`, we can use `encode` directly.
//! // Otherwise, we would need to disambiguate.
//!
//! assert_eq!(12u64.encode(), Ok("12".as_bytes().to_vec()));
//! ```
//!
//! ## Decoding example
//!
//! ```
//! use storey::encoding::{DecodableWithImpl, Encoding, Cover};
//!
//! // - Implementation -
//!
//! struct DisplayEncoding;
//!
//! impl Encoding for DisplayEncoding {
//!    type DecodeError = String;
//!    type EncodeError = String;
//! }
//!
//! impl<T> DecodableWithImpl<DisplayEncoding> for Cover<T>
//! where
//!     T: std::str::FromStr,
//! {
//!     fn decode_impl(data: &[u8]) -> Result<Self, String> {
//!         let string =
//!             String::from_utf8(data.to_vec()).map_err(|_| "string isn't UTF-8".to_string())?;
//!         let value = string.parse().map_err(|_| "parsing failed".to_string())?;
//!         Ok(Cover(value))
//!     }
//! }
//!
//! // - Usage -
//!
//! use storey::encoding::DecodableWith as _;
//!
//! // If there's only one encoding present for `u64`, we can use `decode` directly.
//! // Otherwise, we would need to disambiguate.
//!
//! assert_eq!(u64::decode("12".as_bytes()), Ok(12));
//! ```

/// A trait for types that serve as "markers" for a particular encoding.
/// These types are expected to be empty structs.
pub use storey_encoding::Encoding;

/// A trait for types that can be encoded with a particular encoding.
///
/// # Implementing `EncodableWith`
///
/// The trait is [sealed], so you can't implement it directly. Instead of implementing
/// [`EncodableWith`] for `T`, you should implement [`EncodableWithImpl`] for [`Cover<&T>`].
///
/// [See the module-level documentation for an example.](self)
///
/// [sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
pub use storey_encoding::EncodableWith;

/// A trait for implementing [`EncodableWith`] for a particular encoding.
///
/// This trait exists to allow blanket implementations of [`EncodableWith`] for
/// third-party types. To provide an implementation of [`EncodableWith`] for
/// `MyEncoding` for a type `T`, you should implement [`EncodableWithImpl`] for
/// [`Cover<&T>`]. The reason for this quirky complication are subtleties in Rust's orphan
/// rules.
///
/// [See the module-level documentation for usage.](self)
pub use storey_encoding::EncodableWithImpl;

/// A trait for types that can be decoded with a particular encoding.
///
/// # Implementing `DecodableWith`
///
/// The trait is [sealed], so you can't implement it directly. Instead of implementing
/// [`DecodableWith`] for `T`, you should implement [`DecodableWithImpl`] for [`Cover<T>`].
///
/// [See the module-level documentation for an example.](self)
///
/// [sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
pub use storey_encoding::DecodableWith;

/// A trait for implementing [`DecodableWith`] for a particular encoding.
///
/// This trait exists to allow blanket implementations of [`DecodableWith`] for
/// third-party types. To provide an implementation of [`DecodableWith`] for
/// `MyEncoding` for a type `T`, you should implement [`DecodableWithImpl`] for
/// [`Cover<T>`]. The reason for this quirky complication are subtleties in Rust's orphan
/// rules.
///
/// [See the module-level documentation for usage.](self)
pub use storey_encoding::DecodableWithImpl;

/// A wrapper type used to [cover] type arguments when providing blanket implementations of
/// [`EncodableWithImpl`] and [`DecodableWithImpl`].
///
/// Due to [orphan rules], it is impossible for downstream crates to provide a blanket
/// implementation of [`EncodableWith`] or [`DecodableWith`] for a type parameter `T`.
/// Instead, they should provide a blanket implementation of [`EncodableWithImpl`] or
/// [`DecodableWithImpl`] for `Cover<&T>` or `Cover<T>`, respectively. This ensures that
/// the `T` types are covered and orphan rules are not violated.
///
/// [See the module-level documentation for usage.](self)
///
/// [orphan rules]: https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
/// [cover]: https://doc.rust-lang.org/reference/glossary.html#uncovered-type
pub use storey_encoding::Cover;
