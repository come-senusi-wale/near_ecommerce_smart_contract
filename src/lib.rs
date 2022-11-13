use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::{ U128};
use near_sdk::{env, near_bindgen, AccountId, Promise, CryptoHash};
use near_sdk::serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use crate::metadata::*;
pub use  crate::enummeration::*;
pub use  crate::seller::*;
pub use crate::internal::*;
pub use crate::buyer::*;

mod metadata;
mod enummeration;
mod seller;
mod internal;
mod buyer;

pub type ItemId = u32;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    // keep track of item in list
    pub items_list: UnorderedMap<ItemId, ItemMetadata>,

    // keep track of item buyer add to cart
    pub cart_list: LookupMap<AccountId, UnorderedMap<ItemId, CartMetadata>>,

    //keep track of buyer cost of product in cart
    pub buyer_cost: LookupMap<AccountId, u32>,

    // item id
    pub item_id: u32,
}


/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
   
    TokenPerOwnerInner { account_id_hash: CryptoHash },
   
}

impl Default for Contract {

    fn default() -> Self {
        Self { 
            items_list: UnorderedMap::new(b"c"),
            cart_list: LookupMap::new(b"d"),
            buyer_cost: LookupMap::new(b"e"),
            item_id: 0,
        }
    }
    
}



#[cfg(test)]
mod tests;
