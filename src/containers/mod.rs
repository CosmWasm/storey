use crate::{backend::StorageBackend, encoding::Encoding};

mod item;

pub trait Container<E: Encoding> {
    fn init(&self, storage: &mut impl StorageBackend) -> Result<(), E::EncodeError> {
        Ok(())
    }
}
