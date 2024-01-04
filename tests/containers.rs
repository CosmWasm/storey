mod common;

use stork::containers::{Item, Map};
use stork::StorageBackend as _;

use common::backend::TestStorage;
use common::encoding::TestEncoding;

#[test]
fn item() {
    let storage = TestStorage::new();

    let item0 = Item::<u64, TestEncoding>::new(&[0]);
    let access0 = item0.access();
    access0.set(&storage, &42).unwrap();

    let item1 = Item::<u64, TestEncoding>::new(&[1]);
    let access1 = item1.access();

    assert_eq!(access0.get(&storage).unwrap(), Some(42));
    assert_eq!(storage.get(&[0]), Some(42u64.to_le_bytes().to_vec()));
    assert_eq!(access1.get(&storage).unwrap(), None);
    assert_eq!(storage.get(&[1]), None);
}

#[test]
fn map() {
    let storage = TestStorage::new();

    let map = Map::<str, Item<u64, TestEncoding>, TestEncoding>::new(&[0]);
    let access = map.access();

    access.get("foo").set(&storage, &1337).unwrap();

    assert_eq!(access.get("foo").get(&storage).unwrap(), Some(1337));
    assert_eq!(
        storage.get(&[0, 102, 111, 111]),
        Some(1337u64.to_le_bytes().to_vec())
    );
    assert_eq!(access.get("bar").get(&storage).unwrap(), None);
}

#[test]
fn map_of_map() {
    let storage = TestStorage::new();

    let map = Map::<str, Map<str, Item<u64, TestEncoding>, TestEncoding>, TestEncoding>::new(&[0]);

    map.access()
        .get("foo")
        .get("bar")
        .set(&storage, &1337)
        .unwrap();

    assert_eq!(
        map.access().get("foo").get("bar").get(&storage).unwrap(),
        Some(1337)
    );
    assert_eq!(
        storage.get(&[0, 102, 111, 111, 98, 97, 114]),
        Some(1337u64.to_le_bytes().to_vec())
    );
    assert_eq!(
        map.access().get("foo").get("baz").get(&storage).unwrap(),
        None
    );
}
