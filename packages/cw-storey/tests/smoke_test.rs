use cw_storey::{containers::Item, CwStorage};

use storey::containers::{IterableAccessor as _, Map};

#[test]
fn smoke_test() {
    let mut raw_storage = cosmwasm_std::testing::MockStorage::new();
    let dyn_storage: &mut dyn cosmwasm_std::Storage = &mut raw_storage;
    let mut storage = CwStorage(dyn_storage);

    let item1 = Item::<u64>::new(0);

    item1.access(&mut storage).set(&42).unwrap();
    assert_eq!(item1.access(&mut storage).get().unwrap(), Some(42));

    let item2 = Item::<u64>::new(1);
    assert_eq!(item2.access(&mut storage).get().unwrap(), None);

    assert_eq!(storage.0.get(&[0]), Some(vec![42]));
}

#[test]
fn map() {
    let mut raw_storage = cosmwasm_std::testing::MockStorage::new();
    let dyn_storage: &mut dyn cosmwasm_std::Storage = &mut raw_storage;
    let mut storage = CwStorage(dyn_storage);

    let map = Map::<String, Item<u32>>::new(0);

    map.access(&mut storage).entry_mut("foo").set(&42).unwrap();

    assert_eq!(map.access(&storage).entry("foo").get().unwrap(), Some(42));

    map.access(&mut storage).entry_mut("foo").remove();

    assert_eq!(map.access(&storage).entry("foo").get().unwrap(), None);
}

#[test]
fn iteration() {
    let mut raw_storage = cosmwasm_std::testing::MockStorage::new();
    let dyn_storage: &mut dyn cosmwasm_std::Storage = &mut raw_storage;
    let mut storage = CwStorage(dyn_storage);

    let map = Map::<String, Item<u32>>::new(0);

    map.access(&mut storage).entry_mut("foo").set(&42).unwrap();
    map.access(&mut storage).entry_mut("bar").set(&43).unwrap();

    let access = map.access(&mut storage);
    let mut iter = access.keys();
    assert_eq!(iter.next().unwrap().unwrap().0, "bar");
    assert_eq!(iter.next().unwrap().unwrap().0, "foo");
    assert!(iter.next().is_none());
}
