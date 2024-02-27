mod common;

#[test]
fn storage_backend() {
    // TODO: split this into multiple tests?

    use stork::{IterableStorage as _, RevIterableStorage as _, StorageMut as _};

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

    let some_keys: Vec<_> = storage.keys(Some(&[1]), Some(&[2])).collect();
    assert_eq!(some_keys, vec![vec![1], vec![1, 0], vec![1, 1]]);

    let values: Vec<_> = storage.values(None, None).collect();
    assert_eq!(
        values.iter().collect::<Vec<_>>(),
        vec![&b"bar"[..], b"baz", b"qux", b"quux", b"qux"]
    );

    let some_values: Vec<_> = storage.values(Some(&[1]), Some(&[2])).collect();
    assert_eq!(
        some_values.iter().collect::<Vec<_>>(),
        vec![&b"baz"[..], b"qux", b"quux"]
    );

    let pairs: Vec<_> = storage.pairs(None, None).collect();
    assert_eq!(
        pairs,
        vec![
            (vec![0], b"bar".to_vec()),
            (vec![1], b"baz".to_vec()),
            (vec![1, 0], b"qux".to_vec()),
            (vec![1, 1], b"quux".to_vec()),
            (vec![2], b"qux".to_vec()),
        ]
    );

    let some_pairs: Vec<_> = storage.pairs(Some(&[1]), Some(&[2])).collect();
    assert_eq!(
        some_pairs,
        vec![
            (vec![1], b"baz".to_vec()),
            (vec![1, 0], b"qux".to_vec()),
            (vec![1, 1], b"quux".to_vec()),
        ]
    );

    let rev_keys: Vec<_> = storage.rev_keys(None, None).collect();
    assert_eq!(
        rev_keys,
        vec![vec![2], vec![1, 1], vec![1, 0], vec![1], vec![0]]
    );

    let some_rev_keys: Vec<_> = storage.rev_keys(Some(&[1]), Some(&[2])).collect();
    assert_eq!(some_rev_keys, vec![vec![1, 1], vec![1, 0], vec![1]]);

    let rev_values: Vec<_> = storage.rev_values(None, None).collect();
    assert_eq!(
        rev_values.iter().collect::<Vec<_>>(),
        vec![&b"qux"[..], b"quux", b"qux", b"baz", b"bar"]
    );

    let some_rev_values: Vec<_> = storage.rev_values(Some(&[1]), Some(&[2])).collect();
    assert_eq!(
        some_rev_values.iter().collect::<Vec<_>>(),
        vec![&b"quux"[..], b"qux", b"baz"]
    );

    let rev_pairs: Vec<_> = storage.rev_pairs(None, None).collect();
    assert_eq!(
        rev_pairs,
        vec![
            (vec![2], b"qux".to_vec()),
            (vec![1, 1], b"quux".to_vec()),
            (vec![1, 0], b"qux".to_vec()),
            (vec![1], b"baz".to_vec()),
            (vec![0], b"bar".to_vec()),
        ]
    );

    let some_rev_pairs: Vec<_> = storage.rev_pairs(Some(&[1]), Some(&[2])).collect();
    assert_eq!(
        some_rev_pairs,
        vec![
            (vec![1, 1], b"quux".to_vec()),
            (vec![1, 0], b"qux".to_vec()),
            (vec![1], b"baz".to_vec()),
        ]
    );
}
