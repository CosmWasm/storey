mod common;

#[test]
fn foo() {
    use stork::{StorageBackend as _, StorageIterableBackend as _, StorageRevIterableBackend as _};

    let mut storage = common::backend::TestStorage::new();

    storage.set(&[0], b"bar");
    storage.set(&[1], b"baz");
    storage.set(&[1, 0], b"qux");
    storage.set(&[1, 1], b"quux");
    storage.set(&[2], b"qux");

    let keys: Vec<_> = storage.keys(None, None).collect();
    assert_eq!(
        keys,
        vec![vec![0], vec![1], vec![1, 0], vec![1, 1], vec![2]]
    );

    /*
    let rev_keys: Vec<_> = storage.rev_keys(None, None).collect();
    assert_eq!(
        rev_keys,
        vec![vec![2], vec![1, 1], vec![1, 0], vec![1], vec![0]]
    );
    */

    let keys: Vec<_> = storage.keys(Some(&[1]), Some(&[2])).collect();
    assert_eq!(keys, vec![vec![1], vec![1, 0], vec![1, 1]]);

    /*
    let rev_keys: Vec<_> = storage.rev_keys(Some(&[1]), Some(&[2])).collect();
    assert_eq!(rev_keys, vec![vec![1, 1], vec![1, 0], vec![1]]);
    */
}
