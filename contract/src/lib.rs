use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise};

const DEPOSIT: Balance = 0;
const GAS: Gas = Gas(5_000_000_000_000);
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct contractB {}
impl Default for contractB {
    fn default() -> Self {
        Self {}
    }
}
#[near_bindgen]
impl contractB {
    pub fn call_get_greeting(&self, account_id: AccountId) {
        assert!(
            env::is_valid_account_id(account_id.as_bytes()),
            "this account_id is invalid"
        );
        let a = env::promise_batch_create(&account_id);
        env::promise_batch_action_function_call(a, "get_greeting", b"{}", DEPOSIT, GAS);

        let b = env::promise_batch_then(a, &env::current_account_id());
        env::promise_batch_action_function_call(b, "get_hello", b"{}", DEPOSIT, GAS);

        // let collect = env::promise_and(&[a, b]);
        let finals = env::promise_batch_then(b, &env::current_account_id());
        env::promise_batch_action_function_call(
            finals,
            "love_string",
            b"{}",
            DEPOSIT,
            env::prepaid_gas() * 20,
        );
    }

    pub fn get_hello(&self, #[callback] value: String) -> String {
        return format!(" Xin Chao {}", value);
    }
    pub fn love_string(&self, #[callback] value1: String) -> String {
        return format!("{}. Toi la Tran Van Chinh", value1);
    }
}
