pub mod repay_leverage {
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

    use crate::msg::TokenData;
    use crate::state::{USER_VTOKEN_BALANCE, WRAPPED_TOKEN_BALANCE, WRAPPED_TOKEN_BORROW_BALANCE};
    use crate::ContractError;

    pub fn repay(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_data: TokenData,
    ) -> Result<Response, ContractError> {
        let user_vtoken_balance = match USER_VTOKEN_BALANCE
            .may_load(deps.storage, (&token_data.token_address, &info.sender))
        {
            Ok(opt_data) => match opt_data {
                Some(data) => data,
                None => Uint128::zero(),
            },
            Err(_) => return Err(ContractError::UserVTokenBalanceLoadError {}),
        };

        if user_vtoken_balance.lt(&token_data.token_amount) {
            return Err(ContractError::InsufficientBalance {});
        }

        // Load user's borrow balance
        let wrapped_token_borrow_balance = match WRAPPED_TOKEN_BORROW_BALANCE
            .may_load(deps.storage, (&token_data.token_address, &info.sender))
        {
            Ok(opt_data) => match opt_data {
                Some(data) => data,
                None => Uint128::zero(),
            },
            Err(_) => return Err(ContractError::BorrowBalanceLoadError {}),
        };

        // check if the user's borrow balance is less than the repayment amount
        if wrapped_token_borrow_balance.lt(&token_data.token_amount) {
            return Err(ContractError::RepayOverflow {});
        }

        USER_VTOKEN_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_user_vtoken_balance| -> Result<Uint128, ContractError> {
                match opt_user_vtoken_balance {
                    Some(data) => match data.checked_sub(token_data.token_amount) {
                        Ok(user_vtoken_balance) => Ok(user_vtoken_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => return Err(ContractError::UpdateWrapTokenBorrowErr {}),
                }
            },
        )?;

        // Update user's borrow balance by subtracting the repayment amount
        WRAPPED_TOKEN_BORROW_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_wrapped_token_borrow_balance| -> Result<Uint128, ContractError> {
                match opt_wrapped_token_borrow_balance {
                    Some(data) => match data.checked_sub(token_data.token_amount) {
                        Ok(borror_balance) => Ok(borror_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => return Err(ContractError::UpdateWrapTokenBorrowErr {}),
                }
            },
        )?;

        // Update user's unminted token balance by adding the repayment amount
        WRAPPED_TOKEN_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_wrapped_token_balance| -> Result<Uint128, ContractError> {
                match opt_wrapped_token_balance {
                    Some(data) => match data.checked_add(token_data.token_amount) {
                        Ok(wrapped_token_balance) => Ok(wrapped_token_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => Err(ContractError::UpdateWrapTokenErr {}),
                }
            },
        )?;

        Ok(Response::new().add_attribute("method", "repay"))
    }
}
