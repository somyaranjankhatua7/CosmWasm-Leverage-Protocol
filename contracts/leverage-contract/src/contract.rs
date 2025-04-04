#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::borrow::borrow_leverage;
use crate::burn::burn_tokens;
use crate::deposit::deposit_collateral;
use crate::error::ContractError;
use crate::exchange::exchange_tokens;
use crate::execute::execute_module;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::query_module;
use crate::repay::repay_leverage;
use crate::state::{LEVERAGE_CONTRACT_OWNER, LISTED_TOKEN};
use crate::withdraw::withdraw_collateral;

const CONTRACT_NAME: &str = "crates.io:leverage-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    println!("inst: {}", _msg.token_contract_address);

    let mut token_list: Vec<String> = Vec::new();
    token_list.push(_msg.token_contract_address);
    LISTED_TOKEN.save(deps.storage, &token_list)?;

    LEVERAGE_CONTRACT_OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ListTokenOnLeverage { token_address } => {
            execute_module::list_token_on_leverage(_deps, _env, _info, token_address)
        }
        ExecuteMsg::Receive(cw20_receive_msg) => {
            deposit_collateral::fungible_token(_deps, _env, _info, cw20_receive_msg)
        }
        ExecuteMsg::DepositNative { token_address } => {
            deposit_collateral::native_token(_deps, _env, _info, Addr::unchecked(token_address))
        }
        ExecuteMsg::Borrow(token_data) => borrow_leverage::borrow(_deps, _env, _info, token_data),
        ExecuteMsg::Repay(token_data) => repay_leverage::repay(_deps, _env, _info, token_data),
        ExecuteMsg::Burn(token_data) => burn_tokens::burn(_deps, _env, _info, token_data),
        ExecuteMsg::ExecuteOrder(order_execute) => {
            exchange_tokens::execute_order(_deps, _env, _info, order_execute)
        }
        ExecuteMsg::WithdrawToken(withdraw_data) => {
            withdraw_collateral::withdraw(_deps, _env, _info, withdraw_data)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserCollateralTokenBalance(query_token_data) => to_json_binary(
            &query_module::fetch_user_collateral_token_balance(_deps, _env, query_token_data)?,
        ),
        QueryMsg::UserWrappedTokenBalance(query_token_data) => to_json_binary(
            &query_module::fetch_user_wrapped_token_balance(_deps, _env, query_token_data)?,
        ),
        QueryMsg::UserBorrowTokenBalance(query_token_data) => to_json_binary(
            &query_module::fetch_user_borrow_token_balance(_deps, _env, query_token_data)?,
        ),
        QueryMsg::UserVTokenBalance(query_token_data) => to_json_binary(
            &query_module::fetch_user_v_token_balance(_deps, _env, query_token_data)?,
        ),
        QueryMsg::UserOrders { user_address } => {
            to_json_binary(&query_module::fetch_user_orders(_deps, _env, user_address)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    Ok(Response::new())
}
