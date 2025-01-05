use cosmwasm_std::Deps;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_json_binary, Binary, Env, StdResult};

use crate::msg::QueryResponse;
use crate::state::STATE;

pub fn query_counter(deps: Deps, _env: Env) -> StdResult<Binary> {
    let current_state = STATE.load(deps.storage)?;
    let counter = current_state.counter;

    let resp = to_json_binary(&QueryResponse { counter }).unwrap();
    Ok(resp)
}
#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponse};
    use cosmwasm_std::{to_json_binary, Addr, MessageInfo};
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    const ADDR: &str = "addr1";

    #[test]
    fn test_query() {
        // Initialize dependencies and environment for the test
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Create MessageInfo with a valid sender address
        let info = MessageInfo {
            sender: Addr::unchecked(ADDR), // Create Addr using unchecked for mock testing
            funds: vec![], // No funds being sent
        };

        // Expected response for the counter value 0 and 1
        let expect_data_0 = to_json_binary(&QueryResponse { counter: 0 }).unwrap();
        let expect_data_1 = to_json_binary(&QueryResponse { counter: 1 }).unwrap();

        // Instantiate contract with an empty InstantiateMsg
        let msg = InstantiateMsg {};
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Query once to check the initial counter value
        let msg = QueryMsg::Counter {}; // Query for counter
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap(); // Use deps.as_ref() for query
        assert_eq!(resp, expect_data_0); // Expect the counter to be 0 initially

        // Execute an update action, which should change the counter value
        let msg = ExecuteMsg::Update {}; // Assuming UpdateMsg changes the counter
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Query again to check the updated counter value
        let msg = QueryMsg::Counter {}; // Query for counter again
        let resp = query(deps.as_ref(), env, msg).unwrap(); // Use deps.as_ref() for query
        assert_eq!(resp, expect_data_1); // Expect the counter to be 1 after the update
    }
}

