use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Uint128};
use cw20::{Cw20Coin, Logo, MinterResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::state::OrderState;

#[cw_serde]
pub struct InstantiateMsg {
    pub token_contract_address: String,
}

#[cw_serde]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[cw_serde]
pub struct Cw20Instantiate {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}

#[cw_serde]
pub enum ExecuteMsg {
    ListTokenOnLeverage {
        token_address: String,
    },
    Receive(Cw20ReceiveMsg),
    DepositNative {
        token_address: String,
    },
    Borrow(TokenData),
    ExecuteOrder(OrderExecute),
    Repay(TokenData),
    Burn(TokenData),
    WithdrawToken(WithdrawData)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiquidateData {
    pub liquidate_token_name: Addr,
    pub exchange_token_name: Addr,
    pub liquidate_amount: Uint128,
    pub exchange_amount: Uint128,
    pub user_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenData {
    pub token_address: Addr,
    pub token_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawData {
    pub token_address: Addr,
    pub token_amount: Uint128,
    pub withdraw_type: String,
    pub native: Option<String>,
    pub usdc: Option<String>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}

impl fmt::Display for Cw20ReceiveMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sender:{} amount:{} msg:{}",
            self.sender,
            self.amount,
            self.msg.to_string()
        )
    }
}

#[cw_serde]
pub struct DepositCollateralReceive {
    pub message: String,
    pub ratio: u64,
    pub token_contract: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OrderExecute {
    pub order_id: String,
    pub user_address: Addr,
    pub token_in: Addr,
    pub token_out: Addr,
    pub amount_in: Uint128,
    pub amount_out: Uint128,
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    UserCollateralTokenBalance(QueryTokenData),

    #[returns(Uint128)]
    UserWrappedTokenBalance(QueryTokenData),

    #[returns(Uint128)]
    UserBorrowTokenBalance(QueryTokenData),

    #[returns(Uint128)]
    UserVTokenBalance(QueryTokenData),

    #[returns(OrderState)]
    UserOrders { user_address: Addr }
}



#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct QueryTokenData {
    pub token_address: Addr,
    pub user_address: Addr
}