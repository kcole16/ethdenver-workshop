use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{
    callback_args,
    //    callback_args_vec,
    env,
    ext_contract,
    near_bindgen,
    Promise,
    PromiseOrValue,
};
use serde_json::json;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct CrossContract {}

// If the name is not provided, the namespace for generated methods in derived by applying snake
// case to the trait name, e.g. ext_status_message.
#[ext_contract]
pub trait ExtGuestbook {
    fn addMessage(&mut self, text: String);
    fn getMessages(&self) -> Vec<String>;
}

#[near_bindgen]
impl CrossContract {

    /// Used for callbacks only. Merges two sorted arrays into one. Panics if it is not called by
    /// the contract itself.
    // #[callback_args(data0, data1)]
    // pub fn merge(&self, data0: Vec<u8>, data1: Vec<u8>) -> Vec<u8> {
    //     assert_eq!(env::current_account_id(), env::predecessor_account_id());
    //     self.internal_merge(data0, data1)
    // }

    //    /// Alternative implementation of merge that demonstrates usage of callback_args_vec. Uncomment
    //    /// to use.
    //    #[callback_args_vec(arrs)]
    //    pub fn merge(&self, arrs: &mut Vec<Vec<u8>>) -> Vec<u8> {
    //        assert_eq!(env::current_account_id(), env::predecessor_account_id());
    //        self.internal_merge(arrs.pop().unwrap(), arrs.pop().unwrap())
    //    }

    pub fn simple_call(&mut self, account_id: String, text: String) {
        ext_guestbook::addMessage(text, &account_id, 0, 1000000000000000000);
    }

    pub fn complex_call(&mut self, account_id: String, text: String) -> Promise {
        // 1) call status_message to record a message from the signer.
        // 2) call status_message to retrieve the message of the signer.
        // 3) return that message as its own result.
        // Note, for a contract to simply call another contract (1) is sufficient.
        ext_guestbook::addMessage(text, &account_id, 0, 1000000000000000000).then(
            ext_guestbook::getMessages(
                &account_id,
                0,
                1000000000000000000,
            ),
        )
    }

    // pub fn transfer_money(&mut self, account_id: String, amount: u64) {
    //     Promise::new(account_id).transfer(amount as u128);
    // }
}