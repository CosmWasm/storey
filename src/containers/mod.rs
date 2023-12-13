use crate::encoding::Encoding;

mod item;

pub trait Container<E: Encoding> {
    type Item: EncodableWith<E> + DecodableWith<E>;
}
