use storey::containers::{Column, IterableAccessor as _, Map};

use mocks::backend::TestStorage;
use mocks::encoding::TestEncoding;

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
        .pairs(None, None)
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
