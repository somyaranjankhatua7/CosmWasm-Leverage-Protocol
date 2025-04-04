pub mod borrow_leverage {

    use crate::error::ContractError;
    use crate::msg::TokenData;
    use crate::state::{USER_VTOKEN_BALANCE, WRAPPED_TOKEN_BALANCE, WRAPPED_TOKEN_BORROW_BALANCE};
    
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
    pub fn borrow(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_data: TokenData,
    ) -> Result<Response, ContractError> {
        let wrapped_token = match WRAPPED_TOKEN_BALANCE
            .may_load(deps.storage, (&token_data.token_address, &info.sender))
        {
            Ok(opt_data) => match opt_data {
                Some(data) => data,
                None => Uint128::zero(),
            },
            Err(_) => {
                return Err(ContractError::UnmintedBalanceLoadError {});
            }
        };

        // Check if user's unminted token balance is sufficient
        if wrapped_token.lt(&token_data.token_amount) {
            return Err(ContractError::InsufficientUnmintedToken {});
        }

        // Update user's wrapped token balance by subtracting borrowed amount
        WRAPPED_TOKEN_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_wrapped_token_balance| -> Result<Uint128, ContractError> {
                match opt_wrapped_token_balance {
                    Some(data) => match data.checked_sub(token_data.token_amount) {
                        Ok(wrapped_token_balance) => Ok(wrapped_token_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => Err(ContractError::UpdateWrapTokenErr {}),
                }
            },
        )?;

        // Update user's wrapped borrow balance by adding borrowed amount
        WRAPPED_TOKEN_BORROW_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_borrow_balance| -> Result<Uint128, ContractError> {
                match opt_borrow_balance {
                    Some(data) => match data.checked_add(token_data.token_amount) {
                        Ok(borror_balance) => Ok(borror_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => Ok(token_data.token_amount),
                }
            },
        )?;

        USER_VTOKEN_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_borrow_balance| -> Result<Uint128, ContractError> {
                match opt_borrow_balance {
                    Some(data) => match data.checked_add(token_data.token_amount) {
                        Ok(borror_balance) => Ok(borror_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => Ok(token_data.token_amount),
                }
            },
        )?;
        Ok(Response::new().add_attribute("method", "borrow"))
    }
}
