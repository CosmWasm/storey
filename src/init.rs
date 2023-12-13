use crate::{backend::StorageBackend, encoding::Encoding};

pub trait StorageInit<E>
where
    E: Encoding,
{
    fn init(&self, storage: &mut impl StorageBackend) {}
}
