pub trait Encoding {
    type Type;
    type EncodeError;
    type DecodeError;

    fn encode(v: &Self::Type) -> Result<Vec<u8>, Self::EncodeError>;
    fn decode(data: &[u8]) -> Result<Self::Type, Self::DecodeError>;
}
