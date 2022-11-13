/* unit tests */
#[cfg(test)]
use crate::Contract;
use crate::seller::SellerCore;
use crate::buyer::BuyerCore;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::{env, AccountId};

const MINT_STORAGE_COST: u128 = 100_000_000_000_000_000_000_000;


fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder
}





#[test]
//#[should_panic(expected = "The contract is not initialized")]
fn test_default() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let _contract = Contract::default();
}


#[test]
fn test_add_product_to_list() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::default();

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let name = "rice".to_string();
    let deciption = "fine rice".to_string();
    let price: u32= 30;
    let image = "nice image".to_string();

    contract.add_product(name.clone(), deciption.clone(), price.clone(), image.clone());

    assert_eq!(contract.items_list.len(), 1);
}



#[test]
fn test_edit_product_in_list() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::default();

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let name = "rice".to_string();
    let deciption = "fine rice".to_string();
    let price: u32= 30;
    let image = "nice image".to_string();

    contract.add_product(name.clone(), deciption.clone(), price.clone(), image.clone());

    assert_eq!(contract.items_list.len(), 1);


    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let edit_name = "rice edit".to_string();
    let edit_deciption = "fine rice edit".to_string();
    let edit_price: u32= 30;
    let edit_image:Option<String> = Some("nice image edit".into());
    let item_id:u32 = 1;

    contract.edit_product(item_id, edit_name.clone(), edit_deciption.clone(), edit_price.clone(), edit_image.clone());

    assert_eq!(contract.items_list.len(), 1);
}




#[test]
fn test_delete_product_in_list() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::default();

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let name = "rice".to_string();
    let deciption = "fine rice".to_string();
    let price: u32= 30;
    let image = "nice image".to_string();

    contract.add_product(name.clone(), deciption.clone(), price.clone(), image.clone());

    assert_eq!(contract.items_list.len(), 1);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let two_name = "rice".to_string();
    let two_deciption = "fine rice".to_string();
    let two_price: u32= 30;
    let two_image = "nice image".to_string();

    contract.add_product(two_name.clone(), two_deciption.clone(), two_price.clone(), two_image.clone());

    assert_eq!(contract.items_list.len(), 2);


    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(0)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    
    let item_id:u32 = 1;

    contract.delete_product(item_id);

    assert_eq!(contract.items_list.len(), 1);
}



#[test]
fn test_add_product_to_cart() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::default();

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let name = "rice".to_string();
    let deciption = "fine rice".to_string();
    let price: u32= 30;
    let image = "nice image".to_string();

    contract.add_product(name.clone(), deciption.clone(), price.clone(), image.clone());

    assert_eq!(contract.items_list.len(), 1);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id("akinyemisaheedwale2.testnet".parse().unwrap())
        .build());

    let two_name = "rice".to_string();
    let two_deciption = "fine rice".to_string();
    let two_price: u32= 30;
    let two_image = "nice image".to_string();

    contract.add_product(two_name.clone(), two_deciption.clone(), two_price.clone(), two_image.clone());

    assert_eq!(contract.items_list.len(), 2);


    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());

    
    let item_id:u32 = 1;

    contract.add_to_cart(item_id);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .is_view(true)
        .attached_deposit(0)
        .build());

    let cost = contract.cost(accounts(0));
    assert!(
        cost == price,
        "the cost and price are thesame"
    );

}


 