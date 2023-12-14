mod backend;
mod containers;
mod encoding;

#[cfg(test)]
mod tests {
    // TODO: the reliance on cosmwasm_std is temporary - this should be replaced with our own mocks
    use cosmwasm_std::testing::MockStorage;

    use crate::backend::{
        StorageBackend as _, StorageIterableBackend as _, StorageRevIterableBackend as _,
    };

    #[test]
    fn foo() {
        let mut storage = MockStorage::new();

        storage.set(b"foo", b"bar");
        storage.set(b"baz", b"bax");
        assert_eq!(storage.get(b"foo").unwrap(), b"bar");

        assert!(storage.has(b"foo"));
        assert!(!storage.has(b"foobar"));

        assert_eq!(
            storage.keys(None, None).collect::<Vec<_>>(),
            vec![b"baz", b"foo"],
        );
        assert_eq!(
            storage.rev_keys(None, None).collect::<Vec<_>>(),
            vec![b"foo", b"baz"],
        );

        storage.remove(b"foo");
        assert!(!storage.has(b"foo"));
        assert_eq!(storage.get(b"foo"), None);
    }
}
