use cw_storey::{containers::Item, CwStorage};

use cosmwasm_std::Storage as _;

#[test]
fn smoke_test() {
    let mut storage = CwStorage(cosmwasm_std::testing::MockStorage::new());

    let item1 = Item::<u64>::new(&[0]);

    item1.access(&mut storage).set(&42).unwrap();
    assert_eq!(item1.access(&storage).get().unwrap(), Some(42));

    let item2 = Item::<u64>::new(&[1]);
    assert_eq!(item2.access(&storage).get().unwrap(), None);

    assert_eq!(storage.0.get(&[0]), Some(vec![42]));
}
