use storey::containers::map::Key;
use storey::containers::{Column, Item, IterableAccessor as _, IterableAccessor as _, Map};

use mocks::backend::TestStorage;
use mocks::encoding::TestEncoding;

#[derive(Key)]
struct MyKey(u32);

#[test]
fn key() {
    let mut storage = TestStorage::new();

    let map: Map<MyKey, Item<u64, TestEncoding>> = Map::new(0);

    map.access(&mut storage)
        .entry_mut(&MyKey(1))
        .set(&1337)
        .unwrap();
    map.access(&mut storage)
        .entry_mut(&MyKey(111))
        .set(&133711)
        .unwrap();

    assert_eq!(
        map.access(&storage).entry(&MyKey(1)).get().unwrap(),
        Some(1337)
    );
    assert_eq!(map.access(&storage).entry(&MyKey(0)).get().unwrap(), None);
}

pub struct MyKeySet;
