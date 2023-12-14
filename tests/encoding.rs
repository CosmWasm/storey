use borsh::{BorshDeserialize, BorshSerialize};

use stork::{DecodableWith, EncodableWith, Encoding};

struct Borsh;

impl Encoding for Borsh {
    type EncodeError = std::io::Error;
    type DecodeError = std::io::Error;
}

impl<T> EncodableWith<Borsh> for T
where
    T: BorshSerialize,
{
    fn encode(&self) -> Result<Vec<u8>, Borsh::EncodeError> {
        self.try_to_vec()
    }
}
