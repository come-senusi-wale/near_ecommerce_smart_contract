use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemMetadata {
    pub id: u32,
    pub name: String, 
    pub description: String, 
    pub price: u32, 
    pub image: String,
}


//The Json item is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonItem {
    pub id: u32,
    pub name: String, 
    pub description: String, 
    pub price: u32, 
    pub image: String,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CartMetadata {
    pub id: u32,
    pub qty: u8, 
    pub cost: u32, 
    
}



//The Json cartlist of buyer is what will be returned from view calls. 
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonCartList {
    pub id: u32,
    pub qty: u8, 
    pub cost: u32,
}