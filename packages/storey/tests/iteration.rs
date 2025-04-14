use std::ops::Bound;

use storey::containers::{BoundedIterableAccessor, Item, IterableAccessor as _, Map};

use mocks::backend::TestStorage;
use mocks::encoding::TestEncoding;
use storey_macros::router;

#[test]
fn map_of_map_iteration() {
    let mut storage = TestStorage::new();

    router! {
        router Root {
            0 -> map: Map<String, Map<String, Item<u64, TestEncoding>>>,
        }
    }

    let mut root_access = Root::access(&mut storage);
    let mut access = root_access.map_mut();

    // populate with data
    access.entry_mut("foo").entry_mut("bar").set(&1337).unwrap();
    access.entry_mut("foo").entry_mut("baz").set(&42).unwrap();
    access
        .entry_mut("qux")
        .entry_mut("quux")
        .set(&9001)
        .unwrap();

    // iterate over all items
    let items = access.pairs().collect::<Result<Vec<_>, _>>().unwrap();
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
        .pairs()
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
fn map_of_map_bounded_iteration() {
    let mut storage = TestStorage::new();

    router! {
        router Root {
            0 -> map: Map<String, Map<String, Item<u64, TestEncoding>>>,
        }
    }

    let mut root_access = Root::access(&mut storage);
    let mut access = root_access.map_mut();

    // populate with data
    access.entry_mut("foo").entry_mut("bar").set(&1337).unwrap();
    access.entry_mut("foo").entry_mut("baz").set(&42).unwrap();
    access.entry_mut("foo").entry_mut("qux").set(&9001).unwrap();
    access
        .entry_mut("qux")
        .entry_mut("quux")
        .set(&9001)
        .unwrap();

    // iterate over items under "foo"
    let items = access
        .entry("foo")
        .bounded_pairs(Bound::Unbounded, Bound::Excluded("qux"))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        items,
        vec![
            (("bar".to_string(), ()), 1337),
            (("baz".to_string(), ()), 42),
        ]
    );

    // iterate over items under "foo"
    let items = access
        .entry("foo")
        .bounded_pairs(Bound::Unbounded, Bound::Included("qux"))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        items,
        vec![
            (("bar".to_string(), ()), 1337),
            (("baz".to_string(), ()), 42),
            (("qux".to_string(), ()), 9001),
        ]
    );

    let items = access
        .entry("foo")
        .bounded_pairs(Bound::Excluded("bar"), Bound::Unbounded)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        items,
        vec![
            (("baz".to_string(), ()), 42),
            (("qux".to_string(), ()), 9001),
        ]
    );

    let items = access
        .entry("foo")
        .bounded_pairs(Bound::Included("bar"), Bound::Unbounded)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        items,
        vec![
            (("bar".to_string(), ()), 1337),
            (("baz".to_string(), ()), 42),
            (("qux".to_string(), ()), 9001),
        ]
    );
}
