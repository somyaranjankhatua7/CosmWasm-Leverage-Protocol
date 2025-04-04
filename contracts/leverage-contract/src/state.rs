use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

pub const LEVERAGE_CONTRACT_OWNER: Item<Addr> = Item::new("leverage_contract_owner");
pub const LISTED_TOKEN: Item<Vec<String>> = Item::new("listed_chain");

pub const TOKEN_BALANCE: Map<(&Addr, &Addr), Uint128> = Map::new("user token colateral balance");
pub const WRAPPED_TOKEN_BALANCE: Map<(&Addr, &Addr), Uint128> = Map::new("wrapped token balance");
pub const WRAPPED_TOKEN_BORROW_BALANCE: Map<(&Addr, &Addr), Uint128> =
    Map::new("wrapped token borrow balance");
pub const USER_VTOKEN_BALANCE: Map<(&Addr, &Addr), Uint128> = Map::new("user vtoken balance");

#[cw_serde]
pub struct OrderState {
    pub order_id: String,
    pub sell_token: String,
    pub buy_token: String,
    pub sell_token_amount: Uint128,
    pub buy_token_amount: Uint128,
    pub time: Timestamp,
    pub status: Status,
}

#[cw_serde]
pub enum Status {
    Pending,
    Fullfiled,
    Rejected,
}

pub const ORDER_STATE: Map<&Addr, Vec<OrderState>> = Map::new("order data");