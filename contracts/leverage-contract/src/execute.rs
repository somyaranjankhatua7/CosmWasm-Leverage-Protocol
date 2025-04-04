pub mod execute_module {
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

    use crate::error::ContractError;
    use crate::state::{LEVERAGE_CONTRACT_OWNER, LISTED_TOKEN};

    pub fn list_token_on_leverage(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
    ) -> Result<Response, ContractError> {
        match LEVERAGE_CONTRACT_OWNER.load(deps.storage) {
            Ok(owner) => {
                if owner != info.sender {
                    return Err(ContractError::Unauthorized {});
                }
            }
            Err(err) => {
                return Err(ContractError::GenericError {
                    error: err.to_string(),
                })
            }
        };

        match LISTED_TOKEN.update(
            deps.storage,
            |mut listed_token| -> Result<Vec<String>, ContractError> {
                listed_token.push(token_address);
                Ok(listed_token)
            },
        ) {
            Ok(_) => Ok(Response::new().add_attribute("method", "list_token_on_leverage")),
            Err(_) => Err(ContractError::UpdateTokenListFailed {}),
        }
    }
}
