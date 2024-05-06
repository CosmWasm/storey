use storey::storage::{StorageBackend, StorageBackendMut};

/// A wrapper around a type implementing [`cosmwasm_std::Storage`] that integrates it with [`storey`].
pub struct CwStorage<S>(pub S);

impl<S> StorageBackend for CwStorage<S>
where
    S: cosmwasm_std::Storage,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        cosmwasm_std::Storage::get(&self.0, key)
    }
}

impl<S> StorageBackendMut for CwStorage<S>
where
    S: cosmwasm_std::Storage,
{
    fn set(&mut self, key: &[u8], value: &[u8]) {
        cosmwasm_std::Storage::set(&mut self.0, key, value)
    }

    fn remove(&mut self, key: &[u8]) {
        cosmwasm_std::Storage::remove(&mut self.0, key)
    }
}
