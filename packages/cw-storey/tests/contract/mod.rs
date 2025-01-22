pub mod msg;

use cosmwasm_std::{
    to_json_binary, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};

use cw_storey::containers::{Item, Map};

const ITEM: Item<u32> = Item::new(0);
const MAP: Map<String, Item<u32>> = Map::new(1);

//#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::default())
}

//#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: msg::ExecuteMsg,
) -> Result<Response, StdError> {
    use msg::ExecuteMsg::*;

    match msg {
        SetItem { val } => execute::set_item(deps, val),
        SetMapEntry { key, val } => execute::set_map_entry(deps, key, val),
    }
}

//#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<QueryResponse> {
    use msg::QueryMsg::*;

    match msg {
        Item {} => to_json_binary(&query::get_item(deps)?),
        MapEntry { key } => to_json_binary(&query::get_map_entry(deps, key)?),
        MapEntries {} => to_json_binary(&query::get_map_entries(deps)?),
    }
}

mod execute {
    use super::*;

    pub(crate) fn set_item(deps: DepsMut, val: u32) -> Result<Response, StdError> {
        ITEM.access(deps.storage).set(&val)?;

        Ok(Response::default())
    }

    pub(crate) fn set_map_entry(
        deps: DepsMut,
        key: String,
        val: u32,
    ) -> Result<Response, StdError> {
        MAP.access(deps.storage).entry_mut(&key).set(&val)?;

        Ok(Response::default())
    }
}

mod query {
    use storey::containers::IterableAccessor as _;

    use super::*;

    pub(crate) fn get_item(deps: Deps) -> StdResult<Option<u32>> {
        ITEM.access(deps.storage).get()
    }

    pub(crate) fn get_map_entry(deps: Deps, key: String) -> StdResult<Option<u32>> {
        MAP.access(deps.storage).entry(&key).get()
    }

    pub(crate) fn get_map_entries(deps: Deps) -> StdResult<Vec<(String, u32)>> {
        MAP.access(deps.storage)
            .pairs()
            .map(|res| res.map_err(|e| StdError::generic_err(e.to_string())))
            .map(|res| res.map(|((k, ()), v)| (k, v)))
            .collect()
    }
}
