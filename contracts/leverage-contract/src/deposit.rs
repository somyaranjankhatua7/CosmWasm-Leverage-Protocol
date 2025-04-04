pub mod deposit_collateral {

    use crate::error::ContractError;
    use crate::msg::Cw20ReceiveMsg;
    use crate::state::{LISTED_TOKEN, TOKEN_BALANCE, WRAPPED_TOKEN_BALANCE};

    use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Uint128};

    pub fn native_token(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: Addr,
    ) -> Result<Response, ContractError> {
        if info.funds[0].amount.le(&Uint128::zero()) {
            return Err(ContractError::InsufficientNativeToken {});
        }

        TOKEN_BALANCE.update(
            deps.storage,
            (&token_address, &info.sender),
            |opt_balance| -> Result<Uint128, ContractError> {
                match opt_balance {
                    Some(balance) => match balance.checked_add(info.funds[0].amount) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(ContractError::Overflow {}),
                    },
                    None => Ok(info.funds[0].amount),
                }
            },
        )?;

        // Calculate the wrapped token amount and update the user's wrapped token balance
        let wrapped_token = match info.funds[0].amount.checked_mul(Uint128::from(10u128)) {
            Ok(data) => data,
            Err(_) => return Err(ContractError::Overflow {}),
        };

        WRAPPED_TOKEN_BALANCE.update(
            deps.storage,
            (&token_address, &info.sender),
            |opt_balance| -> Result<Uint128, ContractError> {
                match opt_balance {
                    Some(balance) => match balance.checked_add(wrapped_token) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(ContractError::OverflowBalance {}),
                    },
                    None => Ok(wrapped_token),
                }
            },
        )?;

        Ok(Response::new().add_attribute("method", "deposit_collateral_native"))
    }

    /**
     * @dev Function to handle deposit collateral.
     *
     * This function allows users to deposit tokens into the contract.
     * It performs the following steps:
     * 2. Loads the listed tokens from storage.
     * 3. Checks if the sender's token is listed in the contract.
     * 4. If the token is listed, updates the user's token balance.
     * 5. Calculates the amount of unminted tokens based on the received amount.
     * 6. Updates the user's unminted token balance.
     *
     * @param deps Storage access for contract state.
     * @param _env Contract environment information.
     * @param _info Information about the message sender.
     * @param _cw20_receive_msg CW20 receive message containing sender and amount.
     * @return A response object indicating success or failure.
     */
    pub fn fungible_token(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        let token = match LISTED_TOKEN.load(deps.storage) {
            Ok(tokens) => tokens,
            Err(_) => vec![],
        };

        // Check if the sender's token is listed
        if token.contains(&info.sender.to_string()) {
            println!("{:?}", msg.clone());
            // Update the user's token balance
            TOKEN_BALANCE.update(
                deps.storage,
                (&Addr::unchecked("USDC"), &Addr::unchecked(&msg.sender)),
                |opt_balance| -> Result<Uint128, ContractError> {
                    match opt_balance {
                        Some(balance) => match balance.checked_add(msg.amount) {
                            Ok(data) => Ok(data),
                            Err(_) => Err(ContractError::Overflow {}),
                        },
                        None => Ok(msg.amount),
                    }
                },
            )?;

            // Calculate the wrapped token amount and update the user's wrapped token balance
            let wrapped_token = match msg.amount.checked_mul(Uint128::from(10u128)) {
                Ok(data) => data,
                Err(_) => return Err(ContractError::Overflow {}),
            };

            WRAPPED_TOKEN_BALANCE.update(
                deps.storage,
                (&Addr::unchecked("USDC"), &Addr::unchecked(&msg.sender)),
                |opt_balance| -> Result<Uint128, ContractError> {
                    match opt_balance {
                        Some(balance) => match balance.checked_add(wrapped_token) {
                            Ok(data) => Ok(data),
                            Err(_) => Err(ContractError::OverflowBalance {}),
                        },
                        None => Ok(wrapped_token),
                    }
                },
            )?;
        } else {
            return Err(ContractError::UnauthorizedToken {});
        }

        Ok(Response::new()
            .add_attribute("method", "token_deposit")
            .add_attribute("token_owner", msg.sender)
            .add_attribute("token_address", info.sender))
    }
}
