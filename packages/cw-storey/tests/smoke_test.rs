mod contract;

use cosmwasm_std::testing::{
    message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{from_json, Empty, OwnedDeps};

// The tests in this module are meant to briefly test the integration of `storey`
// with `cosmwasm_std::Storage` and MessagePack serialization.
//
// They're not meant to comprehensively test the storage abstractions provided by `storey`.
// That's already done in the `storey` crate itself. These are "smoke tests".

#[test]
fn item() {
    let mut deps = setup();

    assert_eq!(None, get_item(&deps));
    set_item(&mut deps, 42);
    assert_eq!(Some(42), get_item(&deps));
}

#[test]
fn map() {
    let mut deps = setup();

    assert_eq!(None, get_map_entry(&deps, "foo".to_string()));
    set_map_entry(&mut deps, "foo".to_string(), 42);
    assert_eq!(Some(42), get_map_entry(&deps, "foo".to_string()));
    assert_eq!(None, get_map_entry(&deps, "foobar".to_string()));
}

#[test]
fn iteration() {
    let mut deps = setup();

    set_map_entry(&mut deps, "foo".to_string(), 42);
    set_map_entry(&mut deps, "bar".to_string(), 43);
    set_map_entry(&mut deps, "baz".to_string(), 44);

    assert_eq!(
        vec![
            ("bar".to_string(), 43),
            ("baz".to_string(), 44),
            ("foo".to_string(), 42),
        ],
        get_map_entries(&deps)
    );
}

// The following code provides helper functions to test a mock CosmWasm contract.
// The mock contract itself can be found in the `contract` module.
//
// This kind of setup is common in CosmWasm repos. For example, see the core CosmWasm
// repo: https://github.com/CosmWasm/cosmwasm/tree/main/contracts

fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();
    let creator = deps.api.addr_make("creator");
    let msg = Empty {};
    let info = message_info(&creator, &[]);
    let res = contract::instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    deps
}

#[track_caller]
fn set_item(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>, val: u32) {
    let caller = deps.api.addr_make("caller");
    let msg = contract::msg::ExecuteMsg::SetItem { val };
    contract::execute(deps.as_mut(), mock_env(), message_info(&caller, &[]), msg).unwrap();
}

#[track_caller]
fn set_map_entry(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>, key: String, val: u32) {
    let caller = deps.api.addr_make("caller");
    let msg = contract::msg::ExecuteMsg::SetMapEntry { key, val };
    contract::execute(deps.as_mut(), mock_env(), message_info(&caller, &[]), msg).unwrap();
}

#[track_caller]
fn get_item(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> Option<u32> {
    let res = contract::query(deps.as_ref(), mock_env(), contract::msg::QueryMsg::Item {}).unwrap();
    from_json(&res).unwrap()
}

#[track_caller]
fn get_map_entry(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>, key: String) -> Option<u32> {
    let res = contract::query(
        deps.as_ref(),
        mock_env(),
        contract::msg::QueryMsg::MapEntry { key },
    )
    .unwrap();
    from_json(&res).unwrap()
}

#[track_caller]
fn get_map_entries(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> Vec<(String, u32)> {
    let res = contract::query(
        deps.as_ref(),
        mock_env(),
        contract::msg::QueryMsg::MapEntries {},
    )
    .unwrap();
    from_json(&res).unwrap()
}
