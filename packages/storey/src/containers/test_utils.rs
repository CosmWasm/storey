use std::marker::PhantomData;

use storey_storage::IntoStorage;

use crate::storage::StorageBranch;

use super::Storable;

pub struct BranchContainer<const KEY: u8, T> {
    _phantom: PhantomData<T>,
}

impl<const KEY: u8, T> BranchContainer<KEY, T>
where
    T: Storable,
{
    pub fn access<F, S>(storage: F) -> T::Accessor<StorageBranch<S>>
    where
        (F,): IntoStorage<S>,
    {
        let storage = (storage,).into_storage();

        T::access_impl(StorageBranch::new(storage, vec![KEY]))
    }
}
