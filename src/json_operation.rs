use serde_json::json;

use crate::use_struct::{Users,Address}; 

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
    let parsed = read_json(json);

    println!("Please send a letter to this address street {} , and city {}", parsed.address.street, parsed.address.city );
}


fn read_json(raw_json: &str) -> Users {
    let parsed: Users = serde_json::from_str(raw_json).unwrap();
    return parsed
    
}

pub fn write_json(){
    let user = Users{
        name: String::from("John Doe"),
        age: 43,
        address: Address {
            street: "10 Downing Street".to_string(),
            city: "London".to_string(),
        },
    };

    let json = serde_json::to_string(&user).unwrap();

    println!("{}", json);

}