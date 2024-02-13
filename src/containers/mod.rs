mod item;
mod map;

pub use item::{Item, ItemAccess};
pub use map::{Map, MapAccess};

pub trait Storable {
    type AccessorT<S>;
    type Key;
    type KeyDecodeError;
    type Value;
    type ValueDecodeError;

    fn access_impl<S>(storage: S) -> Self::AccessorT<S>;

    fn decode_key(key: &[u8]) -> Result<Self::Key, ()>;

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError>;
}
