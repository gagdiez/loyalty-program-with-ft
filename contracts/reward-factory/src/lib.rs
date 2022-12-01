use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::Serialize;
use near_sdk::{near_bindgen, AccountId, Balance, Gas};

mod deploy;

const FT_CONTRACT: &[u8] = include_bytes!("../../fungible-token/res/fungible_token.wasm");
const TGAS: Gas = Gas(10u64.pow(12));
const NO_DEPOSIT: Balance = 0;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProgramInfo {
    ft: AccountId,
    manager: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub programs: UnorderedMap<AccountId, ProgramInfo>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            programs: UnorderedMap::new(b"m".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn user_program(&self, account_id: AccountId) -> ProgramInfo {
        match self.programs.get(&account_id) {
            Some(program) => program,
            None => panic!("User has no program"),
        }
    }

    pub fn user_has_program(&self, account_id: AccountId) -> bool {
        match self.programs.get(&account_id) {
            Some(_) => true,
            None => false,
        }
    }
}