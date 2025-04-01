use mocks::encoding::TestEncoding;
use storey::{
    containers::{Item, Map, NonTerminal, Storable},
    storage::StorageBranch,
};
use storey_storage::IntoStorage;

pub struct Foo;

impl Foo {
    pub fn access<F, S>(storage: F) -> FooAccess<StorageBranch<S>>
    where
        (F,): IntoStorage<S>,
    {
        let storage = (storage,).into_storage();
        Self::access_impl(StorageBranch::new(storage, vec![]))
    }
}

impl Storable for Foo {
    type Kind = NonTerminal;
    type Accessor<S> = FooAccess<S>;

    fn access_impl<S>(storage: S) -> Self::Accessor<S> {
        FooAccess { storage }
    }
}

pub struct FooAccess<S> {
    storage: S,
}

impl<S> FooAccess<S> {
    pub fn a(&self) -> <Item<u64, TestEncoding> as Storable>::Accessor<StorageBranch<&S>> {
        Item::access_impl(StorageBranch::new(&self.storage, vec![0]))
    }

    pub fn a_mut(
        &mut self,
    ) -> <Item<u64, TestEncoding> as Storable>::Accessor<StorageBranch<&mut S>> {
        Item::access_impl(StorageBranch::new(&mut self.storage, vec![0]))
    }

    pub fn b(
        &self,
    ) -> <Map<String, Item<u64, TestEncoding>> as Storable>::Accessor<StorageBranch<&S>> {
        Map::access_impl(StorageBranch::new(&self.storage, vec![1]))
    }

    pub fn b_mut(
        &mut self,
    ) -> <Map<String, Item<u64, TestEncoding>> as Storable>::Accessor<StorageBranch<&mut S>> {
        Map::access_impl(StorageBranch::new(&mut self.storage, vec![1]))
    }

    pub fn c(&self) -> <Item<u64, TestEncoding> as Storable>::Accessor<StorageBranch<&S>> {
        Item::access_impl(StorageBranch::new(&self.storage, vec![2]))
    }

    pub fn c_mut(
        &mut self,
    ) -> <Item<u64, TestEncoding> as Storable>::Accessor<StorageBranch<&mut S>> {
        Item::access_impl(StorageBranch::new(&mut self.storage, vec![2]))
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

        let mut map: Map<u32, Foo> = Map::new(0);

        map.access(&mut storage)
            .entry_mut(&0)
            .a_mut()
            .set(&5)
            .unwrap();
        assert_eq!(map.access(&storage).entry(&0).a().get().unwrap(), Some(5));
        assert_eq!(map.access(&storage).entry(&1).a().get().unwrap(), None);
    }
}
