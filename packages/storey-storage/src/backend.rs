use super::storage::{Storage, StorageMut};

/// A trait for immutably accessing a storage backend.
///
/// A collection of basic read operations that can be performed on a storage backend.
///
/// You should only have to interact with this trait if you are implementing a custom storage backend.
pub trait StorageBackend {
    /// Get the value associated with the given key.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Check if the given key exists in the storage backend.
    fn has(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }
}

/// A trait for mutably accessing a storage backend.
///
/// A collection of basic write operations that can be performed on a storage backend.
///
/// You should only have to interact with this trait if you are implementing a custom storage backend.
pub trait StorageBackendMut {
    /// Set the value associated with the given key.
    fn set(&mut self, key: &[u8], value: &[u8]);

    /// Remove the value associated with the given key.
    fn remove(&mut self, key: &[u8]);
}

impl<B> Storage for B
where
    B: StorageBackend,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        StorageBackend::get(self, key)
    }

    fn has(&self, key: &[u8]) -> bool {
        StorageBackend::has(self, key)
    }

    fn get_meta(&self, key: &[u8]) -> Option<Vec<u8>> {
        StorageBackend::get(self, &meta_key(key))
    }

    fn has_meta(&self, key: &[u8]) -> bool {
        StorageBackend::has(self, &meta_key(key))
    }
}

impl<B> StorageMut for B
where
    B: StorageBackendMut,
{
    fn set(&mut self, key: &[u8], value: &[u8]) {
        StorageBackendMut::set(self, key, value)
    }

    fn remove(&mut self, key: &[u8]) {
        StorageBackendMut::remove(self, key)
    }

    fn set_meta(&mut self, key: &[u8], value: &[u8]) {
        StorageBackendMut::set(self, &meta_key(key), value)
    }

    fn remove_meta(&mut self, key: &[u8]) {
        StorageBackendMut::remove(self, &meta_key(key))
    }
}

fn meta_key(key: &[u8]) -> Vec<u8> {
    let mut meta_key = Vec::with_capacity(key.len() + 1);
    meta_key.push(255);
    meta_key.extend_from_slice(key);
    meta_key
}
