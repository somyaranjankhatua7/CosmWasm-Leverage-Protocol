#[cfg(test)]
mod test {
    use cosmwasm_std::{coins, to_json_binary, Addr, Coin, Uint128};
    use cw_multi_test::{AppBuilder, ContractWrapper, Executor};

    use crate::contract::{execute, instantiate, query};
    use crate::execute;
    use crate::msg::{
        Cw20ReceiveMsg, ExecuteMsg, InstantiateMsg, OrderExecute, QueryMsg, QueryTokenData,
        TokenData,
    };

    #[test]
    fn cw_multi_instantiate() {
        let init_funds = coins(10000000000, "uosmo");
        let mut app = AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked("creator"), init_funds)
                .unwrap();
        });

        let group_code_id =
            app.store_code(Box::new(ContractWrapper::new(execute, instantiate, query)));

        let msg = InstantiateMsg {
            token_contract_address: String::from("usdc_contract"),
        };

        let contract_addr = app.instantiate_contract(
            group_code_id,
            Addr::unchecked("creator"),
            &msg,
            &[],
            "leverage_contract",
            None,
        );

        let contract = contract_addr.unwrap();

        let execute_osmo_deposit = ExecuteMsg::DepositNative { token_address: String::from("OSMO") };

        let user_one_execute_send_usdc = app
            .execute_contract(
                Addr::unchecked("creator"),
                contract.clone(),
                &execute_osmo_deposit,
                &[Coin{
                    denom: String::from("uosmo"),
                    amount: Uint128::from(100000000u128)
                }],
            )
            .unwrap();

        // let execute_usdc_deposit_msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        //     sender: String::from("user_one"),
        //     amount: Uint128::from(1000u128),
        //     msg: to_json_binary(&{}).unwrap(),
        // });

        // let user_one_execute_send_usdc = app
        //     .execute_contract(
        //         Addr::unchecked("usdc_contract"),
        //         contract.clone(),
        //         &execute_usdc_deposit_msg,
        //         &[],
        //     )
        //     .unwrap();

        // println!("{:?}", user_one_execute_send_usdc);

        // // --------------------------------------------------------------------------------------
        let user_query_data = QueryTokenData {
            token_address: Addr::unchecked("OSMO"),
            user_address: Addr::unchecked("creator"),
        };

        let res_query_user_collateral_token_balance: Uint128 = app
            .wrap()
            .query_wasm_smart(
                contract.clone(),
                &QueryMsg::UserCollateralTokenBalance(user_query_data.clone()),
            )
            .unwrap();

        println!("collateral: {}", res_query_user_collateral_token_balance)

        // let res_query_user_wrapped_token_balance: Uint128 = app
        //     .wrap()
        //     .query_wasm_smart(
        //         contract.clone(),
        //         &QueryMsg::UserWrappedTokenBalance(user_query_data.clone()),
        //     )
        //     .unwrap();

        // println!(
        //     "res_query_user_collateral_token_balance: {:?}",
        //     res_query_user_collateral_token_balance
        // );
        // println!(
        //     "res_query_user_wrapped_token_balance: {:?}",
        //     res_query_user_wrapped_token_balance
        // );

        // // --------------------------------------------------------------------------------------

        // let user_borrow_token_data = TokenData {
        //     token_address: Addr::unchecked("usdc_contract"),
        //     token_amount: Uint128::from(5000u128),
        // };
        // let borrow_usdc_exe_msg = ExecuteMsg::Borrow(user_borrow_token_data.clone());
        // let borrow_usdc_exe = app
        //     .execute_contract(
        //         Addr::unchecked("user_one"),
        //         contract.clone(),
        //         &borrow_usdc_exe_msg,
        //         &[],
        //     )
        //     .unwrap();

        // println!("{:?}", borrow_usdc_exe);

        // // --------------------------------------------------------------------------------------

        // let res_query_user_wrapped_token_balance: Uint128 = app
        //     .wrap()
        //     .query_wasm_smart(
        //         contract.clone(),
        //         &QueryMsg::UserWrappedTokenBalance(user_query_data.clone()),
        //     )
        //     .unwrap();

        // let res_query_user_borrow_token_balance: Uint128 = app
        //     .wrap()
        //     .query_wasm_smart(
        //         contract.clone(),
        //         &QueryMsg::UserBorrowTokenBalance(user_query_data.clone()),
        //     )
        //     .unwrap();

        // let res_query_user_v_token_balance: Uint128 = app
        //     .wrap()
        //     .query_wasm_smart(
        //         contract.clone(),
        //         &QueryMsg::UserVTokenBalance(user_query_data.clone()),
        //     )
        //     .unwrap();

        // println!(
        //     "res_query_user_wrapped_token_balance: {:?}",
        //     res_query_user_wrapped_token_balance
        // );
        // println!(
        //     "res_query_user_borrow_token_balance: {:?}",
        //     res_query_user_borrow_token_balance
        // );
        // println!(
        //     "res_query_user_v_token_balance: {:?}",
        //     res_query_user_v_token_balance
        // );

        // --------------------------------------------------------------------------------------

        // let order = OrderExecute {
        //     order_id: String::from("id"),
        //     user_address: Addr::unchecked("user_one"),
        //     token_in: Addr::unchecked("usdc_contract"),
        //     amount_in: Uint128::from(1000u128),
        //     token_out: Addr::unchecked("osmo"),
        //     amount_out: Uint128::from(100u128),
        // };

        // let exchange_execute_msg = ExecuteMsg::ExecuteOrder(order);
        // let exchange_order_exe = app.execute_contract(
        //     Addr::unchecked("user_one"),
        //     contract.clone(),
        //     &exchange_execute_msg,
        //     &[],
        // );

        // println!("{:?}", exchange_order_exe);

        // -------------------------------------------------------------------------------------

        // let res_query_user_v_token_balance: Uint128 = app
        // .wrap()
        // .query_wasm_smart(
        //     contract.clone(),
        //     &QueryMsg::UserVTokenBalance(user_query_data.clone()),
        // )
        // .unwrap();

        // let user_query_for_native = QueryTokenData {
        //     token_address: Addr::unchecked("osmo"),
        //     user_address: Addr::unchecked("user_one")
        // };

        // let res_query_user_native_token_balance: Uint128 = app
        // .wrap()
        // .query_wasm_smart(
        //     contract.clone(),
        //     &QueryMsg::UserVTokenBalance(user_query_for_native.clone()),
        // )
        // .unwrap();


        // println!("res_query_user_v_token_balance: {}", res_query_user_v_token_balance);
        // println!("res_query_user_native_token_balance: {}", res_query_user_native_token_balance);
    }
}
