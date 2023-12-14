use borsh::{BorshDeserialize, BorshSerialize};
use stork::{DecodableWithImpl, EncodableWithImpl, Encoding};

struct Borsh;

impl Encoding for Borsh {
    type DecodeError = std::io::Error;
    type EncodeError = std::io::Error;
}

impl<T> EncodableWithImpl<Borsh> for (&T,)
where
    T: BorshSerialize,
{
    fn encode_impl(self) -> Result<Vec<u8>, <Borsh as Encoding>::EncodeError> {
        borsh::to_vec(self.0)
    }
}

impl<T> DecodableWithImpl<Borsh> for (T,)
where
    T: BorshDeserialize,
{
    fn decode_impl(data: &[u8]) -> Result<Self, <Borsh as Encoding>::DecodeError> {
        let item = borsh::from_slice(data)?;
        Ok((item,))
    }
}

#[cfg(test)]
mod tests {
    use stork::{DecodableWith as _, EncodableWith as _};

    #[test]
    fn encoding() {
        let data = (&12u64,).encode().unwrap();
        assert_eq!(data, vec![12, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn decoding() {
        let data = vec![12, 0, 0, 0, 0, 0, 0, 0];
        let item = <u64>::decode(&data).unwrap();
        assert_eq!(item, 12);
    }
}
