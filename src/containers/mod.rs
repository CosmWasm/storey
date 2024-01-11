mod item;
mod map;

pub use item::{Item, ItemAccess};
pub use map::{Map, MapAccess};

use std::borrow::Cow;

use crate::encoding::Encoding;

pub trait Storable<E: Encoding> {
    type AccessorT<'ns>: Accessor;

    fn access_impl<'k>(prefix: impl Into<Cow<'k, [u8]>>) -> Self::AccessorT<'k>;
}

pub trait Collection {
    type Item;
}

pub trait Accessor {}

pub trait Key {
    fn bytes(&self) -> &[u8];
}

impl Key for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
