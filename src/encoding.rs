// TODO: Explain all this. There's a bit much going on here.

pub trait Encoding {
    type EncodeError;
    type DecodeError;
}

pub trait EncodableWith<E: Encoding>: sealed::SealedE<E> {
    fn encode(&self) -> Result<Vec<u8>, E::EncodeError>;
}

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

pub trait DecodableWith<E: Encoding>: Sized + sealed::SealedD<E> {
    fn decode(data: &[u8]) -> Result<Self, E::DecodeError>;
}

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
    use super::*;

    pub trait SealedE<E> {}
    pub trait SealedD<E> {}

    impl<E: Encoding, T> SealedE<E> for T where for<'a> (&'a T,): EncodableWithImpl<E> {}
    impl<E: Encoding, T> SealedD<E> for T where (T,): DecodableWithImpl<E> {}
}
