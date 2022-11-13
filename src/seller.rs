use crate::*;




pub trait SellerCore {

    // add product to list 
    fn add_product(
        &mut self,
        name: String,
        description: String,
        price: u32,
        image: String,
    );


    // for editing product in list
    fn edit_product(
        &mut self, 
        item_id: ItemId,
        name: String,
        description: String,
        price: u32,
        image: Option<String>,
    )-> ItemMetadata;


    // for deleting product
    fn delete_product(&mut self, item_id: ItemId)->ItemMetadata;


    // get information of specific item
    fn item_list(&self, item_id: u32) -> Option<JsonItem>;
}



#[near_bindgen]
impl SellerCore for Contract {

    // add product to list 
    #[payable]
    fn add_product(
        &mut self,
        name: String,
        description: String,
        price: u32,
        image: String,
    ){

        //gettin user id
        let signers = env::predecessor_account_id();

        // get attach deposite 
        let deposite = env::attached_deposit();

        // get initial storagr 
        let initial_sorage = env::storage_usage();
         
        

        assert_eq!(
            self.seller_id(), signers,
            "The seller id  {} is different from the given signer id {}",
            self.seller_id(), signers,
        );

        let item_id:u32 = self.item_id + 1;

        let item = ItemMetadata{

            id: item_id,
            name:  name,
            description: description,
            price:  price,
            image: image,
        };

       

        self.items_list.insert(&item_id, &item);

        self.item_id = item_id;

        self.pay_for_storage(initial_sorage, deposite);
    }


    // edit product in list ??????????
    #[payable]
    fn edit_product(
        &mut self,
        item_id: ItemId,
        name: String,
        description: String,
        price: u32,
        image: Option<String>,
        )-> ItemMetadata{

        //gettin user id
        let signers = env::predecessor_account_id();
        
        // check if the seller id is equal to signer id
        assert_eq!(
        self.seller_id(), signers,
        "The seller id  {} is different from the given signer id {}",
        self.seller_id(), signers,
        );

        //get the item of specific ID
        let mut item = self.items_list.get(&item_id).expect("No item");


        //checking if the image parameter is pass in and update it with respect of all parameter include image else excluding image
        if let Some(image) = image {
            
            item.name = name;
            item.description = description;
            item.price = price;
            item.image = image;
          
        }else{

            item.name = name;
            item.description = description;
            item.price = price;
        }

        self.items_list.insert(&item_id, &item);

        //reture the item that was edited
        item

    }


     // for deleting product
     fn delete_product(&mut self, item_id: ItemId)->ItemMetadata{


        //gettin user id
        let signers = env::predecessor_account_id();

        //get the initial storage space 
        let initial_storage = env::storage_usage();
        
        // check if the seller id is equal to signer id
        assert_eq!(
        self.seller_id(), signers,
        "The seller id  {} is different from the given signer id {}",
        self.seller_id(), signers,
        );

        //get the item of specific ID
        let item = self.items_list.get(&item_id).expect("No item");


        // remove specific item from the list 
        self.items_list.remove(&item_id);

        // reture storage cost that was release to the signer
        self.refund_storage_cost(initial_storage);

        // return item that was deleted
        item
     }



    //get the information for a specific item
    fn item_list(&self, item_id: u32) -> Option<JsonItem> {
        //if there is some item id in the items_list collection
        if let Some(item) = self.items_list.get(&item_id) {
           
            //we return the JsonToken (wrapped by Some since we return an option)
            Some(JsonItem {
                id: item_id,
                name: item.name,
                description: item.description,
                price: item.price, 
                image: item.image,
                
            })
        } else { //if there wasn't a item ID in the items_list collection, we return None
            None
        }
    }


}



