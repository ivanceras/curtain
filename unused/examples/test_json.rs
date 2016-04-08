extern crate rustc_serialize;
extern crate uuid;
extern crate chrono;

use uuid::Uuid;
use std::collections::BTreeMap;
use rustc_serialize::json::{self, Json, ToJson};
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

// Only generate `Decodable` trait implementation
#[derive(RustcDecodable, RustcEncodable)]
//#[derive(RustcDecodable, Debug)]
#[derive(Debug)]
pub struct TestStruct {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
    id:Uuid,
    comment:Option<String>,
    created:DateTime<UTC>,
    updated:Option<DateTime<UTC>>,
}

// Specify encoding method manually
impl ToJson for TestStruct {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        // All standard types implement `to_json()`, so use it
        d.insert("data_int".to_string(), self.data_int.to_json());
        d.insert("data_str".to_string(), self.data_str.to_json());
        d.insert("data_vector".to_string(), self.data_vector.to_json());
        d.insert("id".to_string(), Json::String(self.id.to_string()));
        if self.comment.is_some(){
            d.insert("comment".to_string(), Json::String(self.comment.clone().unwrap().to_string()));
        }
        d.insert("created".to_string(), Json::I64(self.created.timestamp()));
        d.insert("updated".to_string(), Json::I64(self.updated.unwrap().timestamp()));
        Json::Object(d)
    }
}

//TODO: convert Json to TestStruct

fn main() {
    // Serialize using `ToJson`
    let input_data = TestStruct {
        data_int: 1,
        data_str: "madoka".to_string(),
        data_vector: vec![2,3,4,5],
        id:Uuid::new_v4(),
        comment:Some("wolla a hella".to_string()),
        created:UTC::now(),
        updated:Some(UTC::now()),
    };
    let json_obj: Json = input_data.to_json();
    let json_str: String = json_obj.to_string();
    println!("\njson: {}", json_str);

    // Deserialize like before
    let decoded:Json = Json::from_str(&json_str).unwrap();
    println!("\ndecoded: {:?}",decoded);
    //let test_struct: TestStruct = json::decode(&json_str).unwrap();
    //println!("\ntest_struct: {:?}",test_struct);
}
