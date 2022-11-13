use crate::*;
use near_sdk::{CryptoHash};


//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl Contract {
    // the selleer account ID
    pub fn seller_id(&self) -> AccountId {
        let acc = AccountId::from_str("akinyemisaheedwale2.testnet").unwrap();

        acc
    }

    // calculate for storage 

    pub fn pay_for_storage (&self, initial_storage: u64, attach_storage_cost: u128 ){

        // get the current storage 
        let current_storage = env::storage_usage();

        // get the storage used 
        let storage_used = current_storage - initial_storage;

        // get the storage cost per byts
        let storage_cost = env::storage_byte_cost();

        // get payable storage fees 
        if let Some(total_storage_cost) = storage_cost.checked_mul(storage_used as u128)  {

            // checker if user attach enough token to cater for storage 

            assert!(attach_storage_cost > total_storage_cost, "insufficient fund");

            //check for balance 
            let excess_balance = attach_storage_cost - total_storage_cost;

            if excess_balance > 0 {

                self.return_excess_token(excess_balance);
                
            }
            
        }


    }

    // return excess token back to signer

    pub fn return_excess_token(&self, excess_balance: u128){

        // get the addresss
        let signer = env::predecessor_account_id();

        //, Promise

        Promise::new(signer).transfer(excess_balance);
    }


    // calculate storage release
    pub fn refund_storage_cost(&self, initial_storage: u64) {

        // get the signer
        let _signer = env::predecessor_account_id();

        //get the current storage 
        let current_storage = env::storage_usage();

        //compute storage space release 
        let storage_release = initial_storage - current_storage;

        //get the storage unit price 
        let storage_unit_price = env::storage_byte_cost();

        //computing total refungible storage 
        if let Some(refundible_sorage_cost) = storage_unit_price.checked_mul(storage_release.into())  {
            
            //transfer to user wallet address 

            self.return_excess_token(refundible_sorage_cost);
        }else {
            panic!("Error in calculating storage cost");
        }
    }

    
    pub fn customer_product_cost_in_cartlist(&mut self, product_set: UnorderedMap<u32, CartMetadata>, buyer: AccountId){
        let from_index: u32 = 0;
        let limit: u32 = 4;

        //iterate through each product using an iterator
        let price_array: Vec<u32>  = product_set.keys()
            //skip to the index we specified in the start variable
            .skip(from_index as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|item_id| self.customer_cost_list(item_id.clone(), buyer.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect();

        let mut total_cost: u32 = 0;

        for i in price_array.iter() {
            total_cost = total_cost + i;
        }

        //self.buyer_cost.insert(&buyer, &total_cost);

        if let Some(mut cost) = self.buyer_cost.get(&buyer)  {

            cost = total_cost;
            
            self.buyer_cost.insert(&buyer, &cost);
            
        }else {

            self.buyer_cost.insert(&buyer, &total_cost);
            
           
        }
    }

}