mod item;
mod map;

pub use item::{Item, ItemAccess};
pub use map::{Map, MapAccess};

pub trait Storable {
    type AccessorT<S>;

    fn access_impl<S>(storage: S) -> Self::AccessorT<S>;
}
