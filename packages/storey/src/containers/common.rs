#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
pub enum TryGetError<E> {
    #[error("item is empty")]
    Empty,
    #[error(transparent)]
    DecodeError(#[from] E),
}

impl<T: std::fmt::Display> crate::error::StoreyError for TryGetError<T> {}
