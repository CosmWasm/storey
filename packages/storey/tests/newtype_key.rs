use storey::containers::map::{Key, OwnedKey};
use storey::containers::{Item, IterableAccessor as _, Map};

use mocks::backend::TestStorage;
use mocks::encoding::TestEncoding;
use storey_macros::{router, OwnedKey};

#[derive(Key)]
struct MyKey(u32);

#[test]
fn key() {
    let mut storage = TestStorage::new();

    router! {
        router Root {
            0 -> map: Map<MyKey, Item<u64, TestEncoding>>,
        }
    }

    let mut access = Root::access(&mut storage);

    access.map_mut().entry_mut(&MyKey(1)).set(&1337).unwrap();
    access
        .map_mut()
        .entry_mut(&MyKey(111))
        .set(&133711)
        .unwrap();

    assert_eq!(access.map().entry(&MyKey(1)).get().unwrap(), Some(1337));
    assert_eq!(access.map().entry(&MyKey(0)).get().unwrap(), None);
}

#[derive(Key, OwnedKey, Debug, PartialEq)]

pub struct MyOwnedKey(u32);

#[test]
fn owned_key() {
    let mut storage = TestStorage::new();

    router! {
        router Root {
            0 -> map: Map<MyOwnedKey, Item<u64, TestEncoding>>,
        }
    }

    let mut access = Root::access(&mut storage);

    access
        .map_mut()
        .entry_mut(&MyOwnedKey(1))
        .set(&1337)
        .unwrap();
    access
        .map_mut()
        .entry_mut(&MyOwnedKey(111))
        .set(&133711)
        .unwrap();

    let keys = access.map().keys().collect::<Result<Vec<_>, _>>().unwrap();

    assert_eq!(keys, [(MyOwnedKey(1), ()), (MyOwnedKey(111), ())]);
}
