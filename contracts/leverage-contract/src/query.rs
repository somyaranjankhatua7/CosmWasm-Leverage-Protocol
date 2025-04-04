pub mod query_module {
    use cosmwasm_std::{Deps, Env, StdResult, Uint128, Addr};

    use crate::error::ContractError;
    use crate::msg::QueryTokenData;
    use crate::state::{
        TOKEN_BALANCE, USER_VTOKEN_BALANCE, WRAPPED_TOKEN_BALANCE, WRAPPED_TOKEN_BORROW_BALANCE, ORDER_STATE, OrderState
    };

    pub fn fetch_user_collateral_token_balance(
        deps: Deps,
        _env: Env,
        query_token_data: QueryTokenData,
    ) -> StdResult<Uint128> {
        match TOKEN_BALANCE.may_load(
            deps.storage,
            (
                &query_token_data.token_address,
                &query_token_data.user_address,
            ),
        ) {
            Ok(opt_data) => match opt_data {
                Some(data) => Ok(data),
                None => Ok(Uint128::zero()),
            },
            Err(_) => Err(ContractError::UserTokenBalanceQueryFailed {}.into()),
        }
    }

    pub fn fetch_user_wrapped_token_balance(
        deps: Deps,
        _env: Env,
        query_token_data: QueryTokenData,
    ) -> StdResult<Uint128> {
        match WRAPPED_TOKEN_BALANCE.may_load(
            deps.storage,
            (
                &query_token_data.token_address,
                &query_token_data.user_address,
            ),
        ) {
            Ok(opt_data) => match opt_data {
                Some(data) => Ok(data),
                None => Ok(Uint128::zero()),
            },
            Err(_) => Err(ContractError::WrappedTokenQueryFailed {}.into()),
        }
    }

    pub fn fetch_user_borrow_token_balance(
        deps: Deps,
        _env: Env,
        query_token_data: QueryTokenData,
    ) -> StdResult<Uint128> {
        match WRAPPED_TOKEN_BORROW_BALANCE.may_load(
            deps.storage,
            (
                &query_token_data.token_address,
                &query_token_data.user_address,
            ),
        ) {
            Ok(opt_data) => match opt_data {
                Some(data) => Ok(data),
                None => Ok(Uint128::zero()),
            },
            Err(_) => Err(ContractError::UserBorrowTokenBalanceQueryFailed {}.into()),
        }
    }

    pub fn fetch_user_v_token_balance(
        deps: Deps,
        _env: Env,
        query_token_data: QueryTokenData,
    ) -> StdResult<Uint128> {
        match USER_VTOKEN_BALANCE.may_load(
            deps.storage,
            (
                &query_token_data.token_address,
                &query_token_data.user_address,
            ),
        ) {
            Ok(opt_data) => match opt_data {
                Some(data) => Ok(data),
                None => Ok(Uint128::zero()),
            },
            Err(_) => Err(ContractError::UserVTokenBalanceQueryFailed {}.into()),
        }
    }

    pub fn fetch_user_orders(
        deps: Deps,
        _env: Env,
        user_address: Addr
    ) -> StdResult<Vec<OrderState>> {
        match ORDER_STATE.may_load(
            deps.storage,
            &user_address,
        ) {
            Ok(opt_data) => match opt_data {
                Some(data) => Ok(data),
                None => Ok(vec![]),
            },
            Err(_) => Err(ContractError::UserOrderQueryFailed {}.into()),
        }
    }
}
