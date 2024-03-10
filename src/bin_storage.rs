pub trait StorageBackend {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    fn has(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }
}

pub trait StorageBackendMut {
    fn set(&mut self, key: &[u8], value: &[u8]);
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

fn meta_key(key: &[u8]) -> Vec<u8> {
    let mut meta_key = Vec::with_capacity(key.len() + 1);
    meta_key.push(255);
    meta_key.extend_from_slice(key);
    meta_key
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

pub trait Storage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    fn has(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }

    fn get_meta(&self, _key: &[u8]) -> Option<Vec<u8>>;
    fn has_meta(&self, key: &[u8]) -> bool {
        self.get_meta(key).is_some()
    }
}

pub trait StorageMut {
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn remove(&mut self, key: &[u8]);

    fn set_meta(&mut self, _key: &[u8], _value: &[u8]);
    fn remove_meta(&mut self, _key: &[u8]);
}

pub trait IterableStorage {
    type KeysIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type ValuesIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type PairsIterator<'a>: Iterator<Item = (Vec<u8>, Vec<u8>)>
    where
        Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a>;
    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a>;
    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a>;
}

impl<T: IterableStorage> IterableStorage for &T {
    type KeysIterator<'a> = T::KeysIterator<'a> where Self: 'a;
    type ValuesIterator<'a> = T::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = T::PairsIterator<'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        (**self).keys(start, end)
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        (**self).values(start, end)
    }

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        (**self).pairs(start, end)
    }
}

impl<T: IterableStorage> IterableStorage for &mut T {
    type KeysIterator<'a> = T::KeysIterator<'a> where Self: 'a;
    type ValuesIterator<'a> = T::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = T::PairsIterator<'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        (**self).keys(start, end)
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        (**self).values(start, end)
    }

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        (**self).pairs(start, end)
    }
}

pub trait RevIterableStorage {
    type RevKeysIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type RevValuesIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type RevPairsIterator<'a>: Iterator<Item = (Vec<u8>, Vec<u8>)>
    where
        Self: 'a;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a>;
    fn rev_values<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a>;
    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevPairsIterator<'a>;
}
