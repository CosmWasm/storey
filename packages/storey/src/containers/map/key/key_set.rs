/// The default key set for use with a [`Map`](super::super::Map).
///
/// To find out more about key sets, take a look at the [`Key`](super::Key) trait's documentation.
pub struct DefaultKeySet;

pub trait KeySet {}

impl KeySet for DefaultKeySet {}
