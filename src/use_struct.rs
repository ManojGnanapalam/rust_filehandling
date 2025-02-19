use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String
}


#[derive(Serialize, Deserialize)]
pub struct Users {
    pub name: String,
    pub age: i32,
    pub address: Address
}