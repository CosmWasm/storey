mod item;
mod map;

pub use item::{Item, ItemAccess};
pub use map::{Map, MapAccess};

use std::borrow::Cow;

use crate::encoding::Encoding;

pub trait Storable<E: Encoding> {
    type AccessorT<S>;

    fn access_impl<S>(storage: S) -> Self::AccessorT<S>;
}

pub trait Collection {
    type Item;
}

pub trait Key {
    fn bytes(&self) -> &[u8];
}

impl Key for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
