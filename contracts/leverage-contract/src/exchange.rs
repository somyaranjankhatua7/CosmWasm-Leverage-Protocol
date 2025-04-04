pub mod exchange_tokens {
    use crate::error::ContractError;
    use crate::msg::OrderExecute;
    use crate::state::{OrderState, Status, ORDER_STATE, USER_VTOKEN_BALANCE};
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

    pub fn execute_order(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        order: OrderExecute,
    ) -> Result<Response, ContractError> {
        let user_vtoken_balance = match USER_VTOKEN_BALANCE
            .may_load(deps.storage, (&order.token_in, &order.user_address))
        {
            Ok(option_data) => match option_data {
                Some(data) => data,
                None => Uint128::zero(),
            },
            Err(_) => return Err(ContractError::UnableToFetchWrapBorrowToken {}),
        };

        if user_vtoken_balance.lt(&order.amount_in) {
            return Err(ContractError::InsufficientBalance {});
        }

        USER_VTOKEN_BALANCE.update(
            deps.storage,
            (&order.token_out, &order.user_address),
            |opt_data| -> Result<Uint128, ContractError> {
                match opt_data {
                    Some(data) => match data.checked_add(order.amount_out) {
                        Ok(used_wrapped_token_balance) => Ok(used_wrapped_token_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => Ok(order.amount_out),
                }
            },
        )?;

        USER_VTOKEN_BALANCE.update(
            deps.storage,
            (&order.token_in, &order.user_address),
            |opt_data| -> Result<Uint128, ContractError> {
                match opt_data {
                    Some(data) => match data.checked_sub(order.amount_in) {
                        Ok(used_wrapped_token_balance) => Ok(used_wrapped_token_balance),
                        Err(_) => return Err(ContractError::Overflow {}),
                    },
                    None => Ok(order.amount_in),
                }
            },
        )?;

        ORDER_STATE.update(
            deps.storage,
            &order.user_address,
            |opt_data| -> Result<Vec<OrderState>, ContractError> {
                match opt_data {
                    Some(mut data) => {
                        let order_state = OrderState {
                            order_id: order.order_id,
                            sell_token: order.token_in.to_string(),
                            buy_token: order.token_out.to_string(),
                            sell_token_amount: order.amount_in,
                            buy_token_amount: order.amount_out,
                            time: _env.block.time,
                            status: Status::Fullfiled,
                        };
                        data.push(order_state);
                        Ok(data)
                    }
                    None => {
                        let order_state = OrderState {
                            order_id: order.order_id,
                            sell_token: order.token_in.to_string(),
                            buy_token: order.token_out.to_string(),
                            sell_token_amount: order.amount_in,
                            buy_token_amount: order.amount_out,
                            time: _env.block.time,
                            status: Status::Fullfiled,
                        };

                        Ok(vec![order_state])
                    }
                }
            },
        )?;

        Ok(Response::new().add_attribute("method", "execute_order"))
    }
}
