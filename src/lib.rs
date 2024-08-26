use ic_cdk::api::call;
use ic_cdk::export::Principal;
use ic_cdk_macros::{update, query};
use std::collections::HashMap;

#[derive(Default)]
struct Wallet {
    balances: HashMap<Principal, u64>,
}

#[update]
async fn send_tokens(to: Principal, amount: u64) -> Result<(), String> {
    let from = ic_cdk::caller();
    let mut wallet = Wallet::default();
    
    let from_balance = wallet.balances.entry(from).or_insert(0);
    if *from_balance < amount {
        return Err("Insufficient balance".to_string());
    }
    *from_balance -= amount;
    let to_balance = wallet.balances.entry(to).or_insert(0);
    *to_balance += amount;
    
    Ok(())
}

#[query]
fn get_balance() -> u64 {
    let caller = ic_cdk::caller();
    let wallet = Wallet::default();
    *wallet.balances.get(&caller).unwrap_or(&0)
}

