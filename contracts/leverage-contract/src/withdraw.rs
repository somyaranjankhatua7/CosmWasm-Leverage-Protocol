pub mod withdraw_collateral {
    use cosmwasm_std::{
        to_json_binary, BankMsg, Coin, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Response,
        Uint128, WasmMsg,
    };
    
    use crate::error::ContractError;
    use crate::msg::WithdrawData;
    use crate::state::{TOKEN_BALANCE, WRAPPED_TOKEN_BALANCE, WRAPPED_TOKEN_BORROW_BALANCE};

    pub fn withdraw(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        withdraw_data: WithdrawData,
    ) -> Result<Response, ContractError> {
        // Load the user's borrow balance from storage
        let user_borrow_balance = match WRAPPED_TOKEN_BORROW_BALANCE
            .may_load(deps.storage, (&withdraw_data.token_address, &info.sender))
        {
            Ok(opt_data) => match opt_data {
                Some(data) => data,
                None => Uint128::zero(),
            },
            Err(_) => return Err(ContractError::BorrowBalanceLoadError {}),
        };

        // Check if the user has any borrow balance
        if user_borrow_balance.gt(&Uint128::zero()) {
            return Err(ContractError::BorrowAmountIsNotZero {});
        }

        // Load user's token balance
        let token_balance = match TOKEN_BALANCE
            .may_load(deps.storage, (&withdraw_data.token_address, &info.sender))
        {
            Ok(opt_balance) => match opt_balance {
                Some(balance) => balance,
                None => Uint128::zero(),
            },
            Err(_) => {
                return Err(ContractError::GenericError {
                    error: String::from("token balance load error"),
                })
            }
        };

        // Check if user has sufficient balance for withdrawal
        if token_balance.lt(&withdraw_data.token_amount) {
            return Err(ContractError::InsufficientBalance {});
        }

        // Update user's token balance
        TOKEN_BALANCE.update(
            deps.storage,
            (&withdraw_data.token_address, &info.sender),
            |opt_balance| -> Result<Uint128, ContractError> {
                match opt_balance {
                    Some(balance) => match balance.checked_sub(withdraw_data.token_amount) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(ContractError::Overflow {}),
                    },
                    None => Err(ContractError::InsufficientBalance {}),
                }
            },
        )?;

        // Calculate the amount of unminted tokens to remove
        let remove_wrapped_token = match withdraw_data
            .token_amount
            .checked_mul(Uint128::from(10u128))
        {
            Ok(data) => data,
            Err(_) => Uint128::zero(),
        };

        // Update user's unminted token balance
        WRAPPED_TOKEN_BALANCE.update(
            deps.storage,
            (&withdraw_data.token_address, &info.sender),
            |opt_balance| -> Result<Uint128, ContractError> {
                match opt_balance {
                    Some(balance) => match balance.checked_sub(remove_wrapped_token) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(ContractError::Overflow {}),
                    },
                    None => Err(ContractError::InsufficientBalance {}),
                }
            },
        )?;

        let cosmos_msg: CosmosMsg<Empty> =
            if withdraw_data.withdraw_type == String::from("fungible") {
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: withdraw_data.usdc.unwrap(),
                    msg: to_json_binary(&cw20::Cw20ExecuteMsg::Transfer {
                        recipient: info.sender.to_string(),
                        amount: withdraw_data.token_amount,
                    })?,
                    funds: vec![],
                })
            } else if withdraw_data.withdraw_type == String::from("native") {
                CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: vec![Coin {
                        denom: withdraw_data.native.unwrap(),
                        amount: withdraw_data.token_amount,
                    }],
                })
            } else {
                return Err(ContractError::GenericError {
                    error: String::from("Undefined withdraw type"),
                });
            };

        Ok(Response::new()
            .add_attribute("method", "token_withdraw")
            .add_attribute("withdraw_data.token_address", withdraw_data.token_address)
            .add_attribute("user", info.sender)
            .add_message(cosmos_msg))
    }
}
