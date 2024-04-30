use storey::encoding::{Cover, DecodableWithImpl, EncodableWithImpl, Encoding};

/// A simple encoding that uses [*MessagePack*] to encode and decode data.
///
/// This type implements the [`Encoding`] trait (see [`storey::encoding`]), which means it can
/// be used with some of [`storey`]'s containers to encode and decode values.
///
/// You're unlikely to need to use this type directly for basic library usage. You might
/// need it if you're trying to use third-party containers this crate does not provide.
///
/// [*MessagePack*]: https://msgpack.org/
pub struct CwEncoding;

impl Encoding for CwEncoding {
    type DecodeError = ();
    type EncodeError = ();
}

impl<T> EncodableWithImpl<CwEncoding> for Cover<&T>
where
    T: serde::Serialize,
{
    fn encode_impl(self) -> Result<Vec<u8>, ()> {
        rmp_serde::to_vec(self.0).map_err(|_| ())
    }
}

impl<T> DecodableWithImpl<CwEncoding> for Cover<T>
where
    T: serde::de::DeserializeOwned,
{
    fn decode_impl(data: &[u8]) -> Result<Self, ()> {
        rmp_serde::from_slice(data).map(Cover).map_err(|_| ())
    }
}
