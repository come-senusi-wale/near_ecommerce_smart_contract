#!/bin/bash


near delete ecommerce.akinyemisaheedwale2.testnet akinyemisaheedwale2.testnet

near create-account ecommerce.akinyemisaheedwale2.testnet --masterAccount akinyemisaheedwale2.testnet --initialBalance 20

#near create-account ecommerce.akinyemisaheedwale2.testnet --masterAccount akinyemisaheedwale2.testnet 

near deploy --wasmFile=./res/ecommerce.wasm --accountId  ecommerce.akinyemisaheedwale2.testnet