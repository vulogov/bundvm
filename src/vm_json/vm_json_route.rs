extern crate log;
use serde_json;
use easy_error::{Error};

pub fn vm_json_route(obj: serde_json::Map<std::string::String, serde_json::Value>) -> Result<(), Error> {
    println!("{:?}", &obj);
    Ok(())
}
