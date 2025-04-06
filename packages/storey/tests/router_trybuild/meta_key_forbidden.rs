use mocks::encoding::TestEncoding;
use storey::containers::{router, Item, Map};

router! (
    router Foo {
        0 -> a: Item<u64, TestEncoding>,
        1 -> b: Map<String, Item<u64, TestEncoding>>,
        255 -> c: Item<u64, TestEncoding>,
    }
);

fn main() {}
