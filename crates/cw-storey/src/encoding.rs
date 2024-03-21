use storey::encoding::{Cover, DecodableWithImpl, EncodableWithImpl, Encoding};

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
