use cosmwasm_std::StdError;
use thiserror::Error;
use serde::{Serialize, Serializer};

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unauthorized Token")]
    UnauthorizedToken {},

    #[error("Insufficient Balance")]
    InsufficientBalance {},

    #[error("Overflow Balance")]
    OverflowBalance {},

    #[error("Ratio must be smaller than 10")]
    InvalidRatio {},

    #[error("Contract Instantiate Error")]
    ContractInstantiateError {},
    
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Borrow amount is not zero")]
    BorrowAmountIsNotZero {},

    #[error("Calculation Overflow")]
    Overflow {},

    #[error("User Unminted Balance Load Error")]
    UnmintedBalanceLoadError {},

    #[error("Insuffient unminted token for borrowing")]
    InsufficientUnmintedToken {},

    #[error("User Borror Balance Load Error")]
    BorrowBalanceLoadError {},

    #[error("User VToken Balance Load Error")]
    UserVTokenBalanceLoadError {},
    
    #[error("User repay more than borrowed")]
    RepayOverflow {},

    #[error("Pay your borrowed amount")]
    PayBorrowAmount {},

    #[error("User Profit Balance Load Error")]
    ProfitBalanceLoadError {},

    #[error("An error occurred: {error:?}")]
    GenericError { error: String },

    #[error("Update token list failed")]
    UpdateTokenListFailed {},

    #[error("Unable to fetch listed token")]
    UnableToFetchListedToken {}, 

    #[error("Wrapped token query failed")]
    WrappedTokenQueryFailed {},

    #[error("User token balance query failed")]
    UserTokenBalanceQueryFailed {},

    #[error("User borrow token balance query failed")]
    UserBorrowTokenBalanceQueryFailed {},

    #[error("User profit token balance query failed")]
    UserProfitTokenBalanceQueryFailed {},


    #[error("unable to update wrapped token balance")]
    UpdateWrapTokenErr {},

    #[error("unable to update wrapped token borrow balance")]
    UpdateWrapTokenBorrowErr {},

    #[error("Unable to fetch wrapped borrow token balance")]
    UnableToFetchWrapBorrowToken {},

    #[error("Insufficient native tokens")]
    InsufficientNativeToken {},

    #[error("Failed to convert amount")]
    FailedConversion {},

    #[error("User vtoken balance query failed")]
    UserVTokenBalanceQueryFailed {},

    #[error("User orders query failed")]
    UserOrderQueryFailed {}
}

impl Serialize for ContractError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str("ContractError")   
    }
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> StdError {
        StdError::generic_err(err.to_string())
    }
}