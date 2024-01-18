use crate::{Storage, StorageMut};

pub struct StorageBranch<'s, S> {
    backend: &'s S,
    prefix: Vec<u8>,
}

impl<'s, S> StorageBranch<'s, S>
where
    S: Storage,
{
    pub fn new(backend: &'s S, prefix: Vec<u8>) -> Self {
        Self { backend, prefix }
    }
}

impl<S: Storage> Storage for StorageBranch<'_, S> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.backend.get(&[&self.prefix[..], key].concat())
    }
}

impl<S: StorageMut> StorageMut for StorageBranch<'_, S> {
    fn set(&self, key: &[u8], value: &[u8]) {
        self.backend.set(&[&self.prefix[..], key].concat(), value)
    }

    fn remove(&self, key: &[u8]) {
        self.backend.remove(&[&self.prefix[..], key].concat())
    }
}
