use crate::*;

#[near_bindgen]
impl Contract {

    //Query for the total supply of product in  contract
    pub fn items_list_total_supply(&self) -> U128 {
        //return the length of the token metadata by ID
        U128(self.items_list.len() as u128)
    }

    //query for specific product
    pub fn single_item(&self, item_id: u32) -> Option<ItemMetadata> {
        
        self.items_list.get(&item_id)
    }


    //query for pagnition of item
    pub fn items_list(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonItem> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        //let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.items_list.keys()
            //skip to the index we specified in the start variable
            .skip(from_index.unwrap() as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(30) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|item_id| self.item_list(item_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }


    //query for cost
    pub fn cost(&self, account: AccountId)-> u32{
        self.buyer_cost.get(&account).unwrap()

    }


    // get the cart list of particular customer
    pub fn customer_cart_list(&self, account: AccountId, from_index: Option<u32>, limit: Option<u32>)->Vec<JsonCartList>{

        let products = self.cart_list.get(&account);

        let productss = if let Some(products) = products {
            products
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

             //iterate through each token using an iterator
             productss.keys()
             //skip to the index we specified in the start variable
             .skip(from_index.unwrap() as usize) 
             //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
             .take(limit.unwrap_or(30) as usize) 
             //we'll map the token IDs which are strings into Json Tokens
             .map(|item_id| self.product_list(item_id.clone(), account.clone()).unwrap())
             //since we turned the keys into an iterator, we need to turn it back into a vector to return
             .collect()
        

    }
}