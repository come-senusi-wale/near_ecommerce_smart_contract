#!/bin/bash


#add item to list//////////////////////////
near call ecommerce.akinyemisaheedwale2.testnet add_product '{"name": "Some Art", "description": "My NFT media",  "price": 2, "image": "come image"}' --accountId akinyemisaheedwale2.testnet  --deposit 0.1


#view total number of item ////////////////////////
near view ecommerce.akinyemisaheedwale2.testnet items_list_total_supply


#view single item of item ///////////////////////////////
#near view ecommerce.akinyemisaheedwale2.testnet single_item '{"item_id": 2}'


#editng item from list////////////////
#near call ecommerce.akinyemisaheedwale2.testnet edit_product '{"item_id": 2, "name": "Wale Art", "description": "my test",  "price": 300, "image": "edit image"}' --accountId akinyemisaheedwale2.testnet --deposit 0.1



#delete item from product///////////////////////
#near call ecommerce.akinyemisaheedwale2.testnet delete_product '{"item_id": 1}' --accountId akinyemisaheedwale2.testnet


#view pagination of list of item///////////
#near view ecommerce.akinyemisaheedwale2.testnet items_list '{"from_index": 0, "limit": 3}' 

#customer add product to cart ///////////////////////////////
#near call ecommerce.akinyemisaheedwale2.testnet add_to_cart '{"item_id": 2}' --accountId akinyemisaheedwale2.testnet --deposit 0.1

#view pagination of list of product customer add to cart///////////
#near view ecommerce.akinyemisaheedwale2.testnet customer_cart_list '{"account": "akinyemisaheedwale2.testnet", "from_index": 0, "limit": 5}' 


#view customer total cost of product in cart list
#near view ecommerce.akinyemisaheedwale2.testnet cost '{"account": "akinyemisaheedwale2.testnet"}'

#customer increase particular product in cart list ///////////////////////////////
#near call ecommerce.akinyemisaheedwale2.testnet increase_product '{"item_id": 2}' --accountId akinyemisaheedwale4.testnet --deposit 0.1

#customer decrease particular product in cart list ///////////////////////////////
#near call ecommerce.akinyemisaheedwale2.testnet decrease_product '{"item_id": 2}' --accountId akinyemisaheedwale2.testnet --deposit 0.1

#customer remove product in cart list ///////////////////////////////
#near call ecommerce.akinyemisaheedwale2.testnet remove_from_cart '{"item_id": 4}' --accountId akinyemisaheedwale4.testnet 

#customer pay for product ///////////////////////////////
#near call ecommerce.akinyemisaheedwale2.testnet pay_for_product  --accountId akinyemisaheedwale2.testnet --depositYocto 47

#view pagination of list of product customer add to cart///////////
#near view ecommerce.akinyemisaheedwale2.testnet customer_cart_list '{"account": "akinyemisaheedwale2.testnet", "from_index": 0, "limit": 5}' 


#view customer total cost of product in cart list
#near view ecommerce.akinyemisaheedwale2.testnet cost '{"account": "akinyemisaheedwale2.testnet"}'

