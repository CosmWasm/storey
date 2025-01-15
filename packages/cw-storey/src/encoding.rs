use cosmwasm_std::StdError;
use storey::encoding::{Cover, DecodableWithImpl, EncodableWithImpl, Encoding};

/// An encoding that delegates to the [*MessagePack*] encoding provided by the [`cosmwasm_std`] crate.
///
/// This type implements the [`Encoding`] trait (see [`storey::encoding`]), which means it can
/// be used with some of [`storey`]'s containers to encode and decode values.
///
/// You're unlikely to need to use this type directly for basic library usage. You might
/// need it if you're trying to use third-party containers this crate does not provide.
///
/// [*MessagePack*]: https://msgpack.org/
/// [`cosmwasm_std`]: https://docs.rs/cosmwasm-std
pub struct CwEncoding;

impl Encoding for CwEncoding {
    type DecodeError = StdError;
    type EncodeError = StdError;
}

impl<T> EncodableWithImpl<CwEncoding> for Cover<&T>
where
    T: serde::Serialize,
{
    fn encode_impl(self) -> Result<Vec<u8>, StdError> {
        cosmwasm_std::to_msgpack_vec(self.0)
    }
}

impl<T> DecodableWithImpl<CwEncoding> for Cover<T>
where
    T: serde::de::DeserializeOwned,
{
    fn decode_impl(data: &[u8]) -> Result<Self, StdError> {
        cosmwasm_std::from_msgpack(data).map(Cover)
    }
}
