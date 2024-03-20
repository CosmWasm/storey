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
        let mut serialized = Vec::new();
        ciborium::into_writer(&self.0, &mut serialized).map_err(|_| ())?;

        Ok(serialized)
    }
}

impl<T> DecodableWithImpl<CwEncoding> for Cover<T>
where
    T: serde::de::DeserializeOwned,
{
    fn decode_impl(data: &[u8]) -> Result<Self, ()> {
        let value = ciborium::from_reader(data).map_err(|_| ())?;
        Ok(Cover(value))
    }
}
