use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::{ExecuteMsg, InstantiateMsg};

pub mod contract;
pub mod msg;
pub mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    contract::execute(deps, env, info, msg)
}

// study-contract -> build -> contract.wasm
// contract.wasm -> onchain store -> id : 1

// contract - innit -> counter: 0
// contract - execute ->counter ++
// contract - query -> 1

// user A: id: 1 wasm instantiate -> contract - 0x123 -> 0
// user B: 0x123 - query -> 0
// user B: 0x123 - query -> 0

// user A: id: 1 wasm instantiate -> contract - 0x124 -> 0
// user C: 0x124 - execute -> 1
// user B: 0x124 - query -> 1

// user A: id: 1 wasm instantiate -> contract - 0x125
// user A: id: 1 wasm instantiate -> contract - 0x126
