extern crate log;
use crate::vm_json::vm_json_route::{vm_json_route};
use serde_json;
use easy_error::{bail, ensure, Error};

pub fn run_json_instructions(inst: String) -> Result<(), Error> {
    for instruction in inst.lines() {
        match serde_json::from_str::<serde_json::Value>(instruction) {
            Ok(json_inst) => {
                ensure!(json_inst.is_object(), "Invalid JSON type for instruction");
                match json_inst.as_object() {
                    Some(obj) => {
                        match vm_json_route(obj.clone()) {
                            Ok(_) => {},
                            Err(err) => {
                                bail!("{}", err);
                            }
                        }
                    }
                    None => {
                        bail!("JSON: Instruction can not be treated as an object");
                    }
                }
            }
            Err(err) => {
                bail!("VM: error parsing JSON instruction: {}", err);
            }
        }
    }
    Ok(())
}
