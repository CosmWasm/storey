pub enum ExecuteMsg {
    SetItem { val: u32 },
    SetMapEntry { key: String, val: u32 },
}

pub enum QueryMsg {
    Item {},
    MapEntry { key: String },
    MapEntries {},
}
