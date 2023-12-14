use crate::{
    backend::StorageBackend,
    encoding::{DecodableWith, EncodableWith, Encoding},
};

mod item;

pub trait Container<E: Encoding> {
    type Item: EncodableWith<E> + DecodableWith<E>;

    fn init(&self, storage: &mut impl StorageBackend) {}
}
