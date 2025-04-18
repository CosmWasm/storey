use storey_encoding::{Cover, DecodableWithImpl, EncodableWithImpl, Encoding};

// An implementation of an encoding used for tests.
//
// In a real-life scenario, implementers of `EncodableWith` and `DecodableWith`
// will usually provide a blanket implementation that delegates to some third-party
// serialization/deserialization trait. We're imitating this a little here to make
// sure this process works.

pub struct TestEncoding;

#[derive(Debug, PartialEq)]
pub struct MockError;

impl std::fmt::Display for MockError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Encoding for TestEncoding {
    type DecodeError = MockError;
    type EncodeError = MockError;
}

// This is how we would implement `EncodableWith` and `DecodableWith` for
// `MyEncoding`, through a blanket implementation.

impl<T> EncodableWithImpl<TestEncoding> for Cover<&T>
where
    T: MyTestEncoding,
{
    fn encode_impl(self) -> Result<Vec<u8>, <TestEncoding as Encoding>::EncodeError> {
        self.0.my_encode()
    }
}

impl<T> DecodableWithImpl<TestEncoding> for Cover<T>
where
    T: MyTestEncoding,
{
    fn decode_impl(data: &[u8]) -> Result<Self, <TestEncoding as Encoding>::DecodeError> {
        let value = T::my_decode(data)?;
        Ok(Cover(value))
    }
}

// Imagine `MyTestEncoding` is a third-party trait that we don't control.

trait MyTestEncoding: Sized {
    fn my_encode(&self) -> Result<Vec<u8>, MockError>;
    fn my_decode(data: &[u8]) -> Result<Self, MockError>;
}

impl MyTestEncoding for u64 {
    fn my_encode(&self) -> Result<Vec<u8>, MockError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn my_decode(data: &[u8]) -> Result<Self, MockError> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(data);
        Ok(u64::from_le_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use storey_encoding::{DecodableWith as _, EncodableWith as _};

    #[test]
    fn encoding() {
        assert_eq!(12u64.encode(), Ok(12u64.to_le_bytes().to_vec()));
    }

    #[test]
    fn decoding() {
        assert_eq!(<u64>::decode(&12u64.to_le_bytes()), Ok(12));
    }
}
