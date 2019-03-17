#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
mod metrics;
use crate::metrics::{HoldInputData, ConfusionMatrix, Outcome, HoldOutputDataUsize, HoldOutputDataF64, MonitoringInstructions};
use rocket::routes;
use std::collections::HashMap;
use std::sync::Mutex;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;
type MessageMap = Mutex<HashMap<String, HoldInputData>>;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/input")]
fn inputd() -> &'static str {
    "Send a post to /input with the JSON serialized inputs to the model"
}

#[post("/input", format = "json", data = "<message>")]
fn input(message: Json<HashMap<String, f64> >, map: State<MessageMap>) -> JsonValue {
    let instructions=MonitoringInstructions{
        num_new_elements:10
    };
    let original_data=vec![1.0, 1.5, -1.3, 2.0, 1.2, 0.3, 0.2, -0.6, 3.0, 1.2, -0.7];
    let mut hashmap = map.lock().expect("map lock.");
    let mut returnmap:HashMap<String, f64>=HashMap::new();
    for (fieldname, fieldvalue) in &message.0{
        if !hashmap.contains_key(&fieldname){
            let input_data=HoldInputData::new();
            hashmap.insert(fieldname, input_data);
        }
        hashmap.get(&fieldname).push(fieldvalue, &instructions);
        returnmap.get(&fieldname).insert(fieldname, hashmap[fieldname].compute_drift(&original_data));
    }
    json!(returnmap)
}

fn main() {
    rocket::ignite().mount("/", routes![index, inputd, input]).launch();
}