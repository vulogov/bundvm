extern crate log;
use serde_json;
use crate::vm;
use easy_error::{bail, ensure, Error};

pub fn vm_json_call(v: serde_json::Value) -> Result<(), Error> {
    ensure!(v.is_object(), "Invalid JSON type for instruction");
    match v.as_object() {
        Some(o) => {
            match o.get("value") {
                Some(value) => {
                    if value.is_string() {
                        match value.as_str() {
                            Some(val_str) => {
                                let mut vm = vm::BUND.lock().unwrap();
                                vm.call(val_str.to_string());
                                drop(vm);
                            }
                            None => {
                                bail!("Invalid JSON for instruction: Can not cast string for functor name");
                            }
                        }
                    } else {
                        bail!("Invalid JSON for instruction: value not a string");
                    }
                }
                None => {
                    bail!("Invalid JSON for instruction: do not contain a value");
                }
            }
        }
        None => {
            bail!("Invalid JSON for instruction: Can not cast object");
        }
    }
    Ok(())
}
