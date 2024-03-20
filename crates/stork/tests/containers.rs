mod common;

use stork::containers::{Column, Item, Map};
use stork::Storage as _;

use common::backend::TestStorage;
use common::encoding::TestEncoding;

#[test]
fn item() {
    let mut storage = TestStorage::new();

    let item0 = Item::<u64, TestEncoding>::new(&[0]);
    item0.access(&mut storage).set(&42).unwrap();

    let item1 = Item::<u64, TestEncoding>::new(&[1]);
    let access1 = item1.access(&storage);

    assert_eq!(item0.access(&storage).get().unwrap(), Some(42));
    assert_eq!(storage.get(&[0]), Some(42u64.to_le_bytes().to_vec()));
    assert_eq!(access1.get().unwrap(), None);
    assert_eq!(storage.get(&[1]), None);
}

#[test]
fn map() {
    let mut storage = TestStorage::new();

    let map = Map::<String, Item<u64, TestEncoding>>::new(&[0]);

    map.access(&mut storage)
        .entry_mut("foo")
        .set(&1337)
        .unwrap();

    assert_eq!(map.access(&storage).entry("foo").get().unwrap(), Some(1337));
    assert_eq!(
        storage.get(&[0, 3, 102, 111, 111]),
        Some(1337u64.to_le_bytes().to_vec())
    );
    assert_eq!(map.access(&storage).entry("bar").get().unwrap(), None);
}

#[test]
fn map_of_map() {
    let mut storage = TestStorage::new();

    let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(&[0]);

    map.access(&mut storage)
        .entry_mut("foo")
        .entry_mut("bar")
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
    let mut storage = TestStorage::new();

    let map = Map::<String, Item<u64, TestEncoding>>::new(&[0]);
    let mut access = map.access(&mut storage);

    access.entry_mut("foo").set(&1337).unwrap();
    access.entry_mut("bar").set(&42).unwrap();

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
    let mut storage = TestStorage::new();

    let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(&[0]);
    let mut access = map.access(&mut storage);

    // populate with data
    access.entry_mut("foo").entry_mut("bar").set(&1337).unwrap();
    access.entry_mut("foo").entry_mut("baz").set(&42).unwrap();
    access
        .entry_mut("qux")
        .entry_mut("quux")
        .set(&9001)
        .unwrap();

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

#[test]
fn column() {
    let mut storage = TestStorage::new();

    let column = Column::<u64, TestEncoding>::new(&[0]);
    let mut access = column.access(&mut storage);

    access.push(&1337).unwrap();
    access.push(&42).unwrap();

    assert_eq!(access.get(0).unwrap(), Some(1337));
    assert_eq!(access.get(1).unwrap(), Some(42));
    assert_eq!(access.get(2).unwrap(), None);
    assert_eq!(access.len().unwrap(), 2);

    access.remove(0).unwrap();
    assert_eq!(
        access.update(0, &9001),
        Err(stork::containers::column::UpdateError::NotFound)
    );
    access.update(1, &9001).unwrap();

    assert_eq!(access.get(0).unwrap(), None);
    assert_eq!(access.get(1).unwrap(), Some(9001));
    assert_eq!(access.len().unwrap(), 1);
}

#[test]
fn map_of_column() {
    let mut storage = TestStorage::new();

    let map = Map::<String, Column<u64, TestEncoding>>::new(&[0]);
    let mut access = map.access(&mut storage);

    access.entry_mut("foo").push(&1337).unwrap();
    access.entry_mut("foo").push(&42).unwrap();
    access.entry_mut("bar").push(&9001).unwrap();

    assert_eq!(access.entry("foo").get(0).unwrap(), Some(1337));
    assert_eq!(access.entry("foo").get(1).unwrap(), Some(42));
    assert_eq!(access.entry("foo").get(2).unwrap(), None);
    assert_eq!(access.entry("foo").len().unwrap(), 2);

    assert_eq!(access.entry("bar").get(0).unwrap(), Some(9001));
    assert_eq!(access.entry("bar").len().unwrap(), 1);

    let all = access
        .iter(None, None)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        all,
        vec![
            (("bar".to_string(), 0), 9001),
            (("foo".to_string(), 0), 1337),
            (("foo".to_string(), 1), 42)
        ]
    );
}
