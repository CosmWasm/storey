pub trait Encoding {
    /// The error type returned when encoding fails.
    type EncodeError: std::fmt::Display;

    /// The error type returned when decoding fails.
    type DecodeError: std::fmt::Display;
}

pub trait EncodableWith<E: Encoding>: sealed::SealedE<E> {
    fn encode(&self) -> Result<Vec<u8>, E::EncodeError>;
}

pub trait EncodableWithImpl<E: Encoding> {
    fn encode_impl(self) -> Result<Vec<u8>, E::EncodeError>;
}

impl<E: Encoding, T> EncodableWith<E> for T
where
    for<'a> Cover<&'a T>: EncodableWithImpl<E>,
{
    fn encode(&self) -> Result<Vec<u8>, <E as Encoding>::EncodeError> {
        Cover(self).encode_impl()
    }
}

pub trait DecodableWith<E: Encoding>: Sized + sealed::SealedD<E> {
    fn decode(data: &[u8]) -> Result<Self, E::DecodeError>;
}

pub trait DecodableWithImpl<E: Encoding>: Sized {
    fn decode_impl(data: &[u8]) -> Result<Self, E::DecodeError>;
}

impl<E: Encoding, T> DecodableWith<E> for T
where
    Cover<T>: DecodableWithImpl<E>,
{
    fn decode(data: &[u8]) -> Result<Self, <E as Encoding>::DecodeError> {
        let wrapper = <Cover<Self>>::decode_impl(data)?;
        Ok(wrapper.0)
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

    impl<E: Encoding, T> SealedE<E> for T where for<'a> Cover<&'a T>: EncodableWithImpl<E> {}
    impl<E: Encoding, T> SealedD<E> for T where Cover<T>: DecodableWithImpl<E> {}
}

pub struct Cover<T>(pub T);
