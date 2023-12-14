use cosmwasm_std::testing::MockStorage;
use cosmwasm_std::Storage as _;

struct TestStorage(MockStorage);

impl TestStorage {
    pub fn new() -> Self {
        Self(MockStorage::new())
    }
}

impl stork::StorageBackend for TestStorage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        MockStorage::get(&self.0, key)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        MockStorage::set(&mut self.0, key, value)
    }

    fn remove(&mut self, key: &[u8]) {
        MockStorage::remove(&mut self.0, key)
    }
}

impl stork::StorageIterableBackend for TestStorage {
    type KeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type ValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type PairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        self.0.range(start, end, cosmwasm_std::Order::Ascending)
    }

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        self.0
            .range_keys(start, end, cosmwasm_std::Order::Ascending)
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        self.0
            .range_values(start, end, cosmwasm_std::Order::Ascending)
    }
}

impl stork::StorageRevIterableBackend for TestStorage {
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        self.0.range(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a> {
        self.0
            .range_keys(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_values<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        self.0
            .range_values(start, end, cosmwasm_std::Order::Descending)
    }
}
