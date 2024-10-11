use crate::{
    msg::{ExecuteMsg, HelloResp, InstantiateMsg, QueryMsg},
    state::COUNTER,
};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

// instantiate
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;

    Ok(Response::new())
}
// query
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Hello {} => to_binary(&query::hello()?),
        Value {} => to_binary(&query::value(deps)?),
    }
}

mod query {
    use super::*;
    use crate::msg::ValueResp;

    pub fn hello() -> StdResult<HelloResp> {
        let resp = HelloResp {
            message: "Hello World".to_owned(),
        };

        Ok(resp)
    }

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

// execute
#[allow(dead_code)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    use ExecuteMsg::*;

    match msg {
        Increase {} => exec::increase(deps),
    }
}

mod exec {
    use super::*;

    pub fn increase(deps: DepsMut) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;

        counter += 1;
        COUNTER.save(deps.storage, &counter)?;
        // COUNTER.update(deps.storage, |counter| -> StdResult<_> { Ok(counter + 1) })?;

        let resp = Response::new()
            .add_attribute("action", "increase")
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }
}

// test
#[cfg(test)]
mod tests {
    use crate::msg::ValueResp;
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn hello_query() {
        let resp = query::hello().unwrap();
        assert_eq!(
            resp,
            HelloResp {
                message: "Hello World".to_owned()
            }
        );
    }

    #[test]
    fn hello_query_with_app() {
        let mut app: App = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { counter: 0 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: HelloResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Hello {})
            .unwrap();

        assert_eq!(
            resp,
            HelloResp {
                message: "Hello World".to_owned()
            }
        );
        dbg!(resp);
    }

    #[test]
    fn increase() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter: 0 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked("sender"),
                contract_addr.clone(),
                &ExecuteMsg::Increase {},
                &[],
            )
            .unwrap();

        dbg!(resp.events);

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
        dbg!(resp);
    }
}
