// TODO: Explain all this. There's a bit much going on here.

/// A trait for types that serve as "markers" for a particular encoding.
/// These types are expected to be empty structs.
pub trait Encoding {
    type EncodeError;
    type DecodeError;
}

/// A trait for types that can be encoded with a particular encoding.
///
/// # Implementing `EncodableWith`
///
/// The trait is [sealed], so you can't implement it directly. Instead of implementing
/// [`EncodableWith`] for `T`, you should implement [`EncodableWithImpl`] for `(&T,)`.
/// See the documentation for [`EncodableWithImpl`] for an example.
///
/// [sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
pub trait EncodableWith<E: Encoding>: sealed::SealedE<E> {
    fn encode(&self) -> Result<Vec<u8>, E::EncodeError>;
}

/// A trait for implementing [`EncodableWith`] for a particular encoding.
///
/// This trait exists to allow blanket implementations of [`EncodableWith`] for
/// third-party types. To provide an implementation of [`EncodableWith`] for
/// `MyEncoding` for a type `T`, you should implement [`EncodableWithImpl`] for
/// `(&T,)`. The reason for this quirky complication are subtleties in Rust's orphan
/// rules.
///
/// # Examples
///
/// ```
/// use stork::encoding::{EncodableWithImpl, Encoding};
///
/// // Implementation
///
/// struct DisplayEncoding;
///
/// impl Encoding for DisplayEncoding {
///     type DecodeError = ();
///     type EncodeError = ();
/// }
///
/// impl<T> EncodableWithImpl<DisplayEncoding> for (&T,)
/// where
///     T: std::fmt::Display,
/// {
///     fn encode_impl(self) -> Result<Vec<u8>, ()> {
///         Ok(format!("{}", self.0).into_bytes())
///     }
/// }
///
/// // Usage
///
/// use stork::encoding::EncodableWith as _;
///
/// // If there's only one encoding present for `u64`, we can use `encode` directly.
/// // Otherwise, we would need to disambiguate.
///
/// assert_eq!(12u64.encode(), Ok("12".as_bytes().to_vec()));
/// ```
pub trait EncodableWithImpl<E: Encoding> {
    fn encode_impl(self) -> Result<Vec<u8>, E::EncodeError>;
}

impl<E: Encoding, T> EncodableWith<E> for T
where
    for<'a> (&'a T,): EncodableWithImpl<E>,
{
    fn encode(&self) -> Result<Vec<u8>, <E as Encoding>::EncodeError> {
        (self,).encode_impl()
    }
}

/// A trait for types that can be decoded with a particular encoding.
///
/// # Implementing `DecodableWith`
///
/// The trait is [sealed], so you can't implement it directly. Instead of implementing
/// [`DecodableWith`] for `T`, you should implement [`DecodableWithImpl`] for `(T,)`.
/// See the documentation for [`DecodableWithImpl`] for an example.
///
/// [sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
pub trait DecodableWith<E: Encoding>: Sized + sealed::SealedD<E> {
    fn decode(data: &[u8]) -> Result<Self, E::DecodeError>;
}

/// A trait for implementing [`DecodableWith`] for a particular encoding.
///
/// This trait exists to allow blanket implementations of [`DecodableWith`] for
/// third-party types. To provide an implementation of [`DecodableWith`] for
/// `MyEncoding` for a type `T`, you should implement [`DecodableWithImpl`] for
/// `(T,)`. The reason for this quirky complication are subtleties in Rust's orphan
/// rules.
///
/// # Examples
///
/// ```
/// use stork::encoding::{DecodableWithImpl, Encoding};
///
/// // - Implementation -
///
/// struct DisplayEncoding;
///
/// impl Encoding for DisplayEncoding {
///    type DecodeError = ();
///    type EncodeError = ();
/// }
///
/// impl<T> DecodableWithImpl<DisplayEncoding> for (T,)
/// where
///     T: std::str::FromStr,
/// {
///     fn decode_impl(data: &[u8]) -> Result<Self, ()> {
///         let string = String::from_utf8(data.to_vec()).map_err(|_| ())?;
///         let value = string.parse().map_err(|_| ())?;
///         Ok((value,))
///     }
/// }
///
/// // - Usage -
///
/// use stork::encoding::DecodableWith as _;
///
/// // If there's only one encoding present for `u64`, we can use `decode` directly.
/// // Otherwise, we would need to disambiguate.
///
/// assert_eq!(u64::decode("12".as_bytes()), Ok(12));
/// ```
pub trait DecodableWithImpl<E: Encoding>: Sized {
    fn decode_impl(data: &[u8]) -> Result<Self, E::DecodeError>;
}

impl<E: Encoding, T> DecodableWith<E> for T
where
    (T,): DecodableWithImpl<E>,
{
    fn decode(data: &[u8]) -> Result<Self, <E as Encoding>::DecodeError> {
        let tuple = <(Self,)>::decode_impl(data)?;
        Ok(tuple.0)
    }
}

mod sealed {
    // This module is private to the crate. It's used to seal the `EncodableWith` and
    // `DecodableWith` traits, so that the only way they can be implemented outside
    // this crate is through the blanket implementations provided by `EncodableWithImpl`
    // and `DecodableWithImpl`.
    //
    // More information on sealed traits:
    // https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed

    use super::*;

    pub trait SealedE<E> {}
    pub trait SealedD<E> {}

    impl<E: Encoding, T> SealedE<E> for T where for<'a> (&'a T,): EncodableWithImpl<E> {}
    impl<E: Encoding, T> SealedD<E> for T where (T,): DecodableWithImpl<E> {}
}
