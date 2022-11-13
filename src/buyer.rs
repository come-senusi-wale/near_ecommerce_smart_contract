use crate::*;

pub trait BuyerCore {

    //add product to cart
    fn add_to_cart(&mut self, item_id: ItemId);

    // for increasing product in cart list
    fn increase_product(&mut self, item_id: ItemId);

    // for decreasing product in cart list
    fn decrease_product(&mut self, item_id: ItemId);

    // for removing item from cart
    fn remove_from_cart(&mut self, item_id: ItemId);

    //pay for product :::::::::::::::::
    fn pay_for_product(&mut self);


    //for interatble of customer cost
    fn customer_cost_list(&self, item_id: u32, buyer: AccountId)->Option<u32>;


   //for iterable
   fn product_list(&self, item_id: u32, buyer: AccountId)->Option<JsonCartList>;

   //for interable customer payment
   fn customer_pay_list(&self, item_id: u32, buyer: AccountId)->Option<u32>;

}



#[near_bindgen]
impl BuyerCore for Contract {

    //add product to cart
    #[payable]
    fn add_to_cart(&mut self, item_id: ItemId){

         //gettin user id
         let buyer = env::predecessor_account_id();

         // get attach deposite 
         let deposite = env::attached_deposit();
 
         // get initial storagr 
         let initial_sorage = env::storage_usage();

          //get the set of products a buyer add to cart
        let mut product_set = self.cart_list.get(&buyer).unwrap_or_else(|| {
            //if the account doesn't have any item in cartlist, we create a new unorderedMap set
            
           
            UnorderedMap::new(
                StorageKey::TokenPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&buyer),
                }
                .try_to_vec()
                .unwrap(),
            )
           
            
        });



        //check if the item Id already exist and increase the quantities else add new item to cart

        if let Some(mut product) = product_set.get(&item_id)  {
            
            //get the item of specific ID
            let item = self.items_list.get(&item_id).expect("No item");
            
            //get the price of item
            let item_price = item.price;

            let new_qty:u8 = (product.qty + 1).into();

            product.cost = item_price   * new_qty as u32;
            
            product.qty = new_qty;


            product_set.insert(&item_id, &product);
            
        }else {

            //checking the length of product in cart list shouldn't be more four
            assert!(
                product_set.len() <= 3,
                "you can add more than four product to cart list"
            );

            //get the item of specific ID
            let item = self.items_list.get(&item_id).expect("No item");
        
            //get the price of item
            let item_price = item.price;

            let new_product = CartMetadata{

                id: item_id,
                qty:  1,
                cost: item_price,
            };

            product_set.insert(&item_id, &new_product );
            
            
        }

        

        //self.buyer_cost.insert(&buyer, &total_cost);
        self.cart_list.insert(&buyer, &product_set);

        // for calculating customer cart list cost
        self.customer_product_cost_in_cartlist(product_set, buyer);

        
        // pay for storage
        self.pay_for_storage(initial_sorage, deposite);

    }




    // for increasing product in cart list
    #[payable]
    fn increase_product(&mut self, item_id: ItemId){

        //gettin user id
        let buyer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

         
        //get the set of products a buyer add to cart
        let mut product_set = self.cart_list.get(&buyer).expect("no item");


        if let Some(mut product) = product_set.get(&item_id)  {
            
            //get the item of specific ID
            let item = self.items_list.get(&item_id).expect("No item");
            
            //get the price of item
            let item_price = item.price;

            let new_qty:u8 = (product.qty + 1).into();

            product.cost = item_price   * new_qty as u32;
            
            product.qty = new_qty;


            product_set.insert(&item_id, &product);
            
        }else {

            //checking the length of product in cart list shouldn't be more four
            assert!(
                product_set.len() <= 3,
                "you can add more than four product to cart list"
            );

            //get the item of specific ID
            let item = self.items_list.get(&item_id).expect("No item");
        
            //get the price of item
            let item_price = item.price;

            let new_product = CartMetadata{

                id: item_id,
                qty:  1,
                cost: item_price,
            };

            product_set.insert(&item_id, &new_product );
            
            
        }

        //self.buyer_cost.insert(&buyer, &total_cost);
        self.cart_list.insert(&buyer, &product_set);

        // for calculating customer cart list cost
        self.customer_product_cost_in_cartlist(product_set, buyer);


        // get final storage storagr 
        let final_sorage = env::storage_usage();

        // checking the storage 

        if final_sorage > initial_sorage {

            // pay for storage
            self.pay_for_storage(initial_sorage, deposite);
            
        }else{

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

            // get the buyer as the signer again
            let signer = env::predecessor_account_id();

            //also reture back token that initailly deposited
            Promise::new(signer).transfer(deposite);
        }



    }



    // for decreasing product in cart list
    #[payable]
    fn decrease_product(&mut self, item_id: ItemId){

         //gettin user id
         let buyer = env::predecessor_account_id();

         // get attach deposite 
         let deposite = env::attached_deposit();
 
         // get initial storagr 
         let initial_sorage = env::storage_usage();

         
          //get the set of products a buyer add to cart
        let mut product_set = self.cart_list.get(&buyer).expect("no item");


        if let Some(mut product) = product_set.get(&item_id)  {
            
            //get the item of specific ID
            let item = self.items_list.get(&item_id).expect("No item");
            
            //get the price of item
            let item_price = item.price;

            let new_qty:u8 = (product.qty - 1).into();

            product.cost = item_price   * new_qty as u32;
            
            product.qty = new_qty;

             //if the qantity less than 1 we remove it from cart list
            if product.qty < 1 {

                product_set.remove(&item_id);

            } else {
                //if the qannty is not lee than one we inset the  rest . 
                product_set.insert(&item_id, &product);
            }

            
        }


         //if the cart list is empty 
         if product_set.is_empty() {

            self.cart_list.remove(&buyer);

            //remove the buyer from cost list
            self.buyer_cost.remove(&buyer);

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        } else {
            //if the cart  is not empty . 

            //self.buyer_cost.insert(&buyer, &total_cost);
            self.cart_list.insert(&buyer, &product_set);

            // for calculating customer cart list cost
            self.customer_product_cost_in_cartlist(product_set, buyer);

            // get final storage storagr 
            let final_sorage = env::storage_usage();

            // checking the storage 

            if final_sorage > initial_sorage {

                // pay for storage
                self.pay_for_storage(initial_sorage, deposite);
                
            }else{

                // reture storage cost that was release to the buyer
                self.refund_storage_cost(initial_sorage);

                // get the buyer as the signer again
                let signer = env::predecessor_account_id();

                //also reture back token that initailly deposited
                Promise::new(signer).transfer(deposite);
            }

            
            
        }

        

    }



     // for removing item from cart
     
     fn remove_from_cart(&mut self, item_id: ItemId){

        //gettin user id
        let buyer = env::predecessor_account_id();

        // get attach deposite 
        //let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();

        
         //get the set of products a buyer add to cart
       let mut product_set = self.cart_list.get(&buyer).expect("no item");


        if let Some(mut _product) = product_set.get(&item_id)  {
            
            product_set.remove(&item_id);
            
        }

        //if the cart list is empty 
        if product_set.is_empty() {

            self.cart_list.remove(&buyer);

            //remove the buyer from cost list
            self.buyer_cost.remove(&buyer);

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        } else {
            //if the cart  is not empty . 

            //self.buyer_cost.insert(&buyer, &total_cost);
            self.cart_list.insert(&buyer, &product_set);

            // for calculating customer cart list cost
            self.customer_product_cost_in_cartlist(product_set, buyer);

            
            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);
        }


       

     }




     //pay for product :::::::::::::::::
     #[payable]
    fn pay_for_product(&mut self){

        //gettin user id
        let buyer = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        
        let initial_sorage = env::storage_usage();

        
        //get the set of products a buyer add to cart
        let  product_cost = self.buyer_cost.get(&buyer).expect("no product in cart list");

        let total_cost = product_cost as u128;

         //checking ammount attach is equal to or greater than products cost
         assert!(
            deposite > total_cost,
            "you don't have enough money to prochase this goods"
        );


       let seller_id = self.seller_id();

       //pay the seller for selling goods
       Promise::new(seller_id).transfer(total_cost);


        //get the set of products a buyer add to cart
        let mut product_set = self.cart_list.get(&buyer).expect("no item");

        let from_index: u32 = 0;
        let limit: u32 = 4;

        //iterate through each product using an iterator
        let item_id_array: Vec<u32>  = product_set.keys()
            //skip to the index we specified in the start variable
            .skip(from_index as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|item_id| self.customer_pay_list(item_id.clone(), buyer.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect();

        //let mut total_cost: u32 = 0;

        for i in item_id_array.iter() {

            if let Some( _product) = product_set.get(i)  {
            
                product_set.remove(i);
                
            }
        }


        
        //if the cart list is empty 
        if product_set.is_empty() {

            self.cart_list.remove(&buyer);

            //remove the buyer from cost list
            self.buyer_cost.remove(&buyer);

            // reture storage cost that was release to the buyer
            self.refund_storage_cost(initial_sorage);

        }



    }



    //for interatble of customer cost
    fn customer_cost_list(&self, item_id: u32, buyer: AccountId)->Option<u32>{

        if let Some(products_list) = self.cart_list.get(&buyer) {
           
            
            let product = products_list.get(&item_id).unwrap();
            Some(
                product.cost,
            )
        } else { //if there wasn't a item ID in the items_list collection, we return None
            None
        }

    }



    //for interable customer payment
   fn customer_pay_list(&self, item_id: u32, buyer: AccountId)->Option<u32>{
        if let Some(products_list) = self.cart_list.get(&buyer) {
            
            
            let product = products_list.get(&item_id).unwrap();
            Some(
                product.id,
            )
        } else { //if there wasn't a item ID in the items_list collection, we return None
            None
        }
   }


    
    // finction for interable
    fn product_list(&self, item_id: u32, buyer: AccountId)->Option<JsonCartList>{
        //let products_list = self.cart_list.get(&account);

        //let product = products_list.get(&item_id);
        //let buyer = env::predecessor_account_id();

        if let Some(products_list) = self.cart_list.get(&buyer) {
           
            //we return the Jsonitem (wrapped by Some since we return an option)
            let product = products_list.get(&item_id).unwrap();
            Some(JsonCartList {
                id: item_id,
                qty: product.qty,
                cost: product.cost,
            })
        } else { //if there wasn't a item ID in the items_list collection, we return None
            None
        }

    }




}