mod common;

use stork::containers::{Item, Map};
use stork::Storage as _;

use common::backend::TestStorage;
use common::encoding::TestEncoding;

#[test]
fn item() {
    let storage = TestStorage::new();

    let item0 = Item::<u64, TestEncoding>::new(&[0]);
    let access0 = item0.access(&storage);
    access0.set(&42).unwrap();

    let item1 = Item::<u64, TestEncoding>::new(&[1]);
    let access1 = item1.access(&storage);

    assert_eq!(access0.get().unwrap(), Some(42));
    assert_eq!(storage.get(&[0]), Some(42u64.to_le_bytes().to_vec()));
    assert_eq!(access1.get().unwrap(), None);
    assert_eq!(storage.get(&[1]), None);
}

#[test]
fn map() {
    let storage = TestStorage::new();

    let map = Map::<String, Item<u64, TestEncoding>>::new(&[0]);
    let access = map.access(&storage);

    access.get("foo").set(&1337).unwrap();

    assert_eq!(access.get("foo").get().unwrap(), Some(1337));
    assert_eq!(
        storage.get(&[0, 102, 111, 111]),
        Some(1337u64.to_le_bytes().to_vec())
    );
    assert_eq!(access.get("bar").get().unwrap(), None);
}

#[test]
fn map_of_map() {
    let storage = TestStorage::new();

    let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(&[0]);

    map.access(&storage)
        .get("foo")
        .get("bar")
        .set(&1337)
        .unwrap();

    assert_eq!(
        map.access(&storage).get("foo").get("bar").get().unwrap(),
        Some(1337)
    );
    assert_eq!(
        storage.get(&[0, 102, 111, 111, 98, 97, 114]),
        Some(1337u64.to_le_bytes().to_vec())
    );
    assert_eq!(
        map.access(&storage).get("foo").get("baz").get().unwrap(),
        None
    );
}
