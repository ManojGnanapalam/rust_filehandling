use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String
}


#[derive(Serialize, Deserialize)]
struct Users {
    name: String,
    age: i32,
    address: Address
}

pub fn json_handling() {
    let json = r#"
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    }
}
"#;
    let parsed = read_json_typed(json);

    println!("Please send a letter to this address street {} , and city {}", parsed.address.street, parsed.address.city );
}


fn read_json_typed(raw_json: &str) -> Users {
    let parsed: Users = serde_json::from_str(raw_json).unwrap();
    return parsed
    
}