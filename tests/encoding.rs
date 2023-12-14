use borsh::{BorshDeserialize, BorshSerialize};

use stork::Encoding;

struct Borsh<T>(T);

impl<T> Encoding for Borsh<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    type Type = T;
    type EncodeError = std::io::Error;
    type DecodeError = std::io::Error;

    fn encode(v: &Self::Type) -> Result<Vec<u8>, Self::EncodeError> {
        borsh::to_vec(v)
    }

    fn decode(data: &[u8]) -> Result<Self::Type, Self::DecodeError> {
        borsh::from_slice(data)
    }
}
