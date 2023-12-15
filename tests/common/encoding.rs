use stork::{DecodableWithImpl, EncodableWithImpl, Encoding};

pub struct TestEncoding;

impl Encoding for TestEncoding {
    type DecodeError = ();
    type EncodeError = ();
}

impl<T> EncodableWithImpl<TestEncoding> for (&T,)
where
    T: MyEncoding,
{
    fn encode_impl(self) -> Result<Vec<u8>, <TestEncoding as Encoding>::EncodeError> {
        self.0.my_encode()
    }
}

impl<T> DecodableWithImpl<TestEncoding> for (T,)
where
    T: MyEncoding,
{
    fn decode_impl(data: &[u8]) -> Result<Self, <TestEncoding as Encoding>::DecodeError> {
        let value = T::my_decode(data)?;
        Ok((value,))
    }
}

trait MyEncoding: Sized {
    fn my_encode(&self) -> Result<Vec<u8>, ()>;
    fn my_decode(data: &[u8]) -> Result<Self, ()>;
}

impl MyEncoding for u64 {
    fn my_encode(&self) -> Result<Vec<u8>, ()> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn my_decode(data: &[u8]) -> Result<Self, ()> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(data);
        Ok(u64::from_le_bytes(bytes))
    }
}
