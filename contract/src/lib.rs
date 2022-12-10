// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Gas, Promise, PromiseError};

mod external;
mod internal;

pub use crate::external::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DepositProxy {
    depository_account: AccountId,
}

// Implement the contract structure
#[near_bindgen]
impl DepositProxy {
    #[init]
    pub fn new(depository_account: AccountId) -> Self {
        Self::internal_new(depository_account)
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate(depository_account: AccountId) -> Self {
        Self { depository_account }
    }

    #[payable]
    pub fn deposit(&mut self) -> Promise {
        self.internal_deposit()
    }

    #[private]
    pub fn query_deposit_callback(
        &self,
        #[callback_result] call_result: Result<bool, PromiseError>,
        signer_account_id: AccountId,
        attached_deposit: Balance,
    ) -> String {
        if call_result.is_err() {
            log!("There was an error contacting Depository");
            log!(
                "Return attached deposit {} to signer {}",
                attached_deposit,
                signer_account_id,
            );
            Promise::new(signer_account_id).transfer(attached_deposit);
            return "Unsuccess".to_string();
        }

        if call_result.unwrap() {
            log!("Success");
            "Success".to_string()
        } else {
            log!("Unsuccess");
            "Unsuccess".to_string()
        }
    }
}

impl Default for DepositProxy {
    fn default() -> Self {
        env::panic_str("Contract should be initialized before usage")
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    // use super::*;
}
