

use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AddressBook {
    pub total_supply: u64,
    pub tokens: BTreeMap<u64, Principal>,
    pub controllers: Vec<Principal>,
    pub claim_index: u64,
    pub token_seeds: BTreeMap<u64, u64>
}


impl AddressBook {

    pub fn user_tokens(&self, user: &Principal) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();
        for (token_id, user_id) in &self.tokens {
            if user_id == user {
                results.push(*token_id);
            }
        }
        return results;
    }

    pub fn owner_of(&self, token_id: &u64) -> Option<Principal> {
        match self.tokens.get(token_id) {
            Some(owner_id) => return Some(owner_id.clone()),
            None => {
                return None;
            }
        }
    }

    pub fn is_owner_of(&self, user: Principal, token_id: &u64) -> bool {
        match self.owner_of(&token_id) {
            Some(owner_id) => return user == owner_id,
            None => {
                return false;
            }
        }
    }

    pub fn is_controller(&self, user: &Principal) -> bool {
        return self.controllers.contains(user);
    }

    pub fn add_controller(&mut self, user: &Principal) -> bool {
        if !self.is_controller(user) {
            self.controllers.push(user.clone());
            return true;
        }
        return false;
    }

    pub fn remove_controller(&mut self, user: &Principal) -> bool {
        if self.is_controller(user) {
            let index = self.controllers.iter().position(|x| x == user).unwrap();
            self.controllers.remove(index);
            return true;
        }
        return false;
    }

    pub fn undo_transfer(&mut self, user_id: Principal, token_id: u64) -> bool {
        if let Some(token_owner) = self.tokens.get(&token_id) {
            if &user_id == token_owner {
                self.tokens.insert(token_id, ic_cdk::caller());
                return true;
            } 
        }
        return false;
    }

    pub fn transfer_to(&mut self, user: Principal, token_id: u64) -> bool {
        if let Some(token_owner) = self.tokens.get(&token_id) {
            if token_owner == &ic_cdk::caller() {
                self.tokens.insert(token_id, user);
                return true;
            }
        }
        return false;
    }

    pub fn remaining(&self) -> u64 {
        return self.total_supply - self.claim_index;
    }

    pub fn is_claimed(&self, token_id: &u64) -> bool {
        return self.tokens.contains_key(token_id);
    }

    pub fn claim(&mut self, user_id: Principal) -> Result<u64, String> {

        if self.claim_index >= self.total_supply {
            return Err("No more claims left".to_string());
        }

        self.claim_index += 1;
    
        match self.tokens.get(&self.claim_index) {
            Some(_) => return Err("Already claimed".to_string()),
            None => {
                self.token_seeds.insert(self.claim_index, ic_cdk::api::time() as u64);
                self.tokens.insert(self.claim_index, user_id);
                return Ok(self.claim_index);
            }
        }
    }
}