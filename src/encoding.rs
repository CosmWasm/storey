// TODO: I figure this trait might improve error messages when someone tries
//       to use a type that isn't a marker of a serialization format, but I'm not sure
//       if it's really necessary. We should determine that!
pub trait Encoding {}

pub trait EncodableWith<E: Encoding> {
    type EncodeError;

    fn encode(&self) -> Result<Vec<u8>, Self::EncodeError>;
}

pub trait DecodableWith<E: Encoding>: Sized {
    type DecodeError;

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError>;
}
