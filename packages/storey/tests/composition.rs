use storey::containers::{Column, Item, IterableAccessor as _, Map};

use mocks::backend::TestStorage;
use mocks::encoding::TestEncoding;
use storey_storage::Storage as _;

#[test]
fn map_of_map() {
    let mut storage = TestStorage::new();

    let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(0);

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
fn map_of_column() {
    let mut storage = TestStorage::new();

    let map = Map::<String, Column<u64, TestEncoding>>::new(0);
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

    let all = access.pairs().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(
        all,
        vec![
            (("bar".to_string(), 0), 9001),
            (("foo".to_string(), 0), 1337),
            (("foo".to_string(), 1), 42)
        ]
    );
}
