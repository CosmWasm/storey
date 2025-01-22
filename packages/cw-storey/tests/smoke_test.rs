use cw_storey::containers::Item;

use storey::containers::{IterableAccessor as _, Map};
use storey::storage::IntoStorage as _;

// The tests in this module are meant to briefly test the integration of `storey`
// with `cosmwasm_std::Storage` and MessagePack serialization.
//
// They're not meant to comprehensively test the storage abstractions provided by `storey`.
// That's already done in the `storey` crate itself.

// this mimicks how storage is accessed from CosmWasm smart contracts,
// where developers have access to `&dyn cosmwasm_std::Storage` or
// `&mut dyn cosmwasm_std::Storage`, but not the concrete type.
fn give_me_storage() -> Box<dyn cosmwasm_std::Storage> {
    Box::new(cosmwasm_std::testing::MockStorage::new())
}

#[test]
fn smoke_test() {
    let mut storage = give_me_storage();

    let item1 = Item::<u64>::new(0);

    item1.access(&mut *storage).set(&42).unwrap();
    assert_eq!(item1.access(&mut *storage).get().unwrap(), Some(42));

    let item2 = Item::<u64>::new(1);
    assert_eq!(item2.access(&mut *storage).get().unwrap(), None);

    assert_eq!((&mut *storage,).into_storage().0.get(&[0]), Some(vec![42]));
}

#[test]
fn map() {
    let mut storage = give_me_storage();

    let map = Map::<String, Item<u32>>::new(0);

    map.access(&mut *storage).entry_mut("foo").set(&42).unwrap();

    assert_eq!(
        map.access(&mut *storage).entry("foo").get().unwrap(),
        Some(42)
    );
}

#[test]
fn iteration() {
    let mut storage = give_me_storage();

    let map = Map::<String, Item<u32>>::new(0);

    map.access(&mut *storage).entry_mut("foo").set(&42).unwrap();
    map.access(&mut *storage).entry_mut("bar").set(&43).unwrap();

    let access = map.access(&mut *storage);
    let mut iter = access.keys();
    assert_eq!(iter.next().unwrap().unwrap().0, "bar");
    assert_eq!(iter.next().unwrap().unwrap().0, "foo");
    assert!(iter.next().is_none());
}
