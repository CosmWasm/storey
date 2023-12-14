pub trait Encoding {
    type EncodeError;
    type DecodeError;
}

pub trait EncodableWith<E: Encoding> {
    fn encode(&self) -> Result<Vec<u8>, E::EncodeError>;
}

pub trait DecodableWith<E: Encoding>: Sized {
    fn decode(data: &[u8]) -> Result<Self, E::DecodeError>;
}
