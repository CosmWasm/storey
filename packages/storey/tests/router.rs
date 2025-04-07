use mocks::encoding::TestEncoding;
use storey::containers::{Item, Map};
use storey_macros::router;

router! {
    router Foo {
        0 -> a: Item<u64, TestEncoding>,
        1 -> b: Map<String, Item<u64, TestEncoding>>,
        2 -> c: Item<u64, TestEncoding>,
    }
}

#[cfg(test)]
mod tests {
    use mocks::backend::TestStorage;

    use super::*;

    #[test]
    fn set_and_get() {
        let mut storage = TestStorage::new();

        let mut foo = Foo::access(&mut storage);
        foo.a_mut().set(&5).unwrap();

        assert_eq!(foo.a().get().unwrap(), Some(5));
        assert_eq!(foo.c().get().unwrap(), None);

        foo.b_mut().entry_mut("key").set(&10).unwrap();
        assert_eq!(foo.b().entry("key").get().unwrap(), Some(10));
    }

    #[test]
    fn nested() {
        let mut storage = TestStorage::new();

        let map: Map<u32, Foo> = Map::new(0);

        map.access(&mut storage)
            .entry_mut(&0)
            .a_mut()
            .set(&5)
            .unwrap();
        assert_eq!(map.access(&storage).entry(&0).a().get().unwrap(), Some(5));
        assert_eq!(map.access(&storage).entry(&1).a().get().unwrap(), None);
    }

    #[test]
    fn trybuild() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/router_trybuild/*.rs");
    }
}
