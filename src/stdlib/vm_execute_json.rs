extern crate log;
use crate::vm_json::vm_json_route::{vm_json_route};
use serde_json;
use easy_error::{bail, Error};

pub fn run_json_instructions(inst: String) -> Result<(), Error> {
    for instruction in inst.lines() {
        match serde_json::from_str::<serde_json::Value>(instruction) {
            Ok(json_inst) => {
                match vm_json_route(json_inst.clone()) {
                    Ok(_) => {},
                    Err(err) => {
                        bail!("{}", err);
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
