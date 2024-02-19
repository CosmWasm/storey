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

    access.entry("foo").set(&1337).unwrap();

    assert_eq!(access.entry("foo").get().unwrap(), Some(1337));
    assert_eq!(
        storage.get(&[0, 3, 102, 111, 111]),
        Some(1337u64.to_le_bytes().to_vec())
    );
    assert_eq!(access.entry("bar").get().unwrap(), None);
}

#[test]
fn map_of_map() {
    let storage = TestStorage::new();

    let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(&[0]);

    map.access(&storage)
        .entry("foo")
        .entry("bar")
        .set(&1337)
        .unwrap();

    assert_eq!(
        map.access(&storage)
            .entry("foo")
            .entry("bar")
            .get()
            .unwrap(),
        Some(1337)
    );
    assert_eq!(
        storage.get(&[0, 3, 102, 111, 111, 3, 98, 97, 114]),
        Some(1337u64.to_le_bytes().to_vec())
    );
    assert_eq!(
        map.access(&storage)
            .entry("foo")
            .entry("baz")
            .get()
            .unwrap(),
        None
    );
}

#[test]
fn simple_iteration() {
    let storage = TestStorage::new();

    let map = Map::<String, Item<u64, TestEncoding>>::new(&[0]);
    let access = map.access(&storage);

    access.entry("foo").set(&1337).unwrap();
    access.entry("bar").set(&42).unwrap();

    let items = access
        .iter(None, None)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        items,
        vec![
            (("bar".to_string(), ()), 42),
            (("foo".to_string(), ()), 1337)
        ]
    );
}

#[test]
fn composable_iteration() {
    let storage = TestStorage::new();

    let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(&[0]);
    let access = map.access(&storage);

    // populate with data
    access.entry("foo").entry("bar").set(&1337).unwrap();
    access.entry("foo").entry("baz").set(&42).unwrap();
    access.entry("qux").entry("quux").set(&9001).unwrap();

    // iterate over all items
    let items = access
        .iter(None, None)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        items,
        vec![
            (("foo".to_string(), ("bar".to_string(), ())), 1337),
            (("foo".to_string(), ("baz".to_string(), ())), 42),
            (("qux".to_string(), ("quux".to_string(), ())), 9001)
        ]
    );

    // iterate over items under "foo"
    let items = access
        .entry("foo")
        .iter(None, None)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        items,
        vec![
            (("bar".to_string(), ()), 1337),
            (("baz".to_string(), ()), 42)
        ]
    );
}
