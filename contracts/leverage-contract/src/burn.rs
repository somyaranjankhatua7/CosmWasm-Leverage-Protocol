pub mod burn_tokens {

    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

    use crate::error::ContractError;
    use crate::msg::TokenData;
    use crate::state::{TOKEN_BALANCE, USER_VTOKEN_BALANCE, WRAPPED_TOKEN_BALANCE, WRAPPED_TOKEN_BORROW_BALANCE};
    /**
     * Function to burn vToken and receive underlying assets.
     *
     * This function allows users to burn vTokens and receive the underlying assets in return.
     * It performs the following steps:
     * 1. Loads the user's borrow balance and checks if it is greater than zero.
     * 2. If the borrow balance is greater than zero, returns an overflow error indicating that the borrow balance must be zero for burning.
     * 3. Loads the user's profit balance and checks if it is sufficient for the burning.
     * 4. If the profit balance is less than the vToken amount, returns an error indicating insufficient balance.
     * 5. Calculates the amount of underlying USDC tokens to be received based on the vToken amount.
     * 6. Updates the user's token balance by adding the received USDC amount.
     * 7. Updates the user's unminted token balance by adding the vToken amount.
     * 8. Returns a response indicating success.
     *
     * @param _deps Storage access for contract state.
     * @param _env Contract environment information.
     * @param _info Information about the message sender.
     * @param _token_data.token_address Address of the vToken to be burned.
     * @param _v_token_data.token_amount Amount of vTokens to be burned.
     * @return A response object indicating success or failure.
     */

    pub fn burn(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_data: TokenData
    ) -> Result<Response, ContractError> {
        // Load the user's borrow balance from storage
        let user_borrow_balance = match WRAPPED_TOKEN_BORROW_BALANCE.may_load(deps.storage, (&token_data.token_address, &info.sender)) {
            Ok(opt_data) => match opt_data {
                Some(data) => data,
                None => Uint128::zero(),
            },
            Err(_) => return Err(ContractError::BorrowBalanceLoadError {}),
        };

        // If user's borrow balance is greater than zero, return an error
        if user_borrow_balance.gt(&Uint128::zero()) {
            return Err(ContractError::PayBorrowAmount {});
        }

        // Load the user's profit balance from storage
        let user_vtoken_balance =
            match USER_VTOKEN_BALANCE.may_load(deps.storage, (&token_data.token_address, &info.sender)) {
                Ok(opt_data) => match opt_data {
                    Some(data) => data,
                    None => Uint128::zero(),
                },
                Err(_) => return Err(ContractError::ProfitBalanceLoadError {}),
            };

        if user_vtoken_balance.lt(&token_data.token_amount) {
            return Err(ContractError::InsufficientBalance {})
        }

        // Calculate the equivalent USDC amount based on the VToken amount burned (assuming 10:1 ratio)
        let user_collateral_amount = match token_data.token_amount.checked_div(Uint128::from(10u128)) {
            Ok(data) => data,
            Err(_) => return Err(ContractError::Overflow {}),
        };
        

        // Update user's token balance by adding the calculated USDC amount
        TOKEN_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_balance| -> Result<Uint128, ContractError> {
                match opt_balance {
                    Some(balance) => match balance.checked_add(user_collateral_amount) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(ContractError::Overflow {}),
                    },
                    None => Ok(user_collateral_amount),
                }
            },
        )?;

        // Update user's unminted token balance by adding the VToken amount burned
        WRAPPED_TOKEN_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_balance| -> Result<Uint128, ContractError> {
                match opt_balance {
                    Some(balance) => match balance.checked_add(token_data.token_amount) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(ContractError::Overflow {}),
                    },
                    None => Ok(token_data.token_amount),
                }
            },
        )?;

        USER_VTOKEN_BALANCE.update(
            deps.storage,
            (&token_data.token_address, &info.sender),
            |opt_balance| -> Result<Uint128, ContractError> {
                match opt_balance {
                    Some(balance) => match balance.checked_sub(token_data.token_amount) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(ContractError::Overflow {}),
                    },
                    None => return Err(ContractError::GenericError { error: String::from("doesn't have v tokens") }),
                }
            }
        )?;

        Ok(Response::new())
    }
}
