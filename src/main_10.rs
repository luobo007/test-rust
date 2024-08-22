
use serde::{Serialize, Deserialize};
use serde_json::json;


fn main(){
   
    let  address =Address{
        street:String::from("zhujiang new trown"),
        city:String::from("guangzhou")
    };


   let mut _user = User {
    name:String::from("Daniel"),
    age:18,
    email:String::from("123@gmail.com"),
    address:address,
    phone_numbers:[String::from("135111111"),String::from("13611111")]
   };

   let serialized = serde_json::to_string(&_user).unwrap();
   println!("Serialized: {}", serialized);
}

#[derive(Serialize)]
struct User {
    name:String,
    age:u8,
    email:String,
    address:Address,
    phone_numbers:[String;2]
}


#[derive(Serialize)]
struct Address {
    street:String,
    city:String
}