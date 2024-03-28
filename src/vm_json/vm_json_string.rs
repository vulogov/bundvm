extern crate log;
use serde_json;
use crate::vm;
use rust_dynamic::value;
use easy_error::{bail, ensure, Error};

pub fn vm_json_numbers_push_string(v: serde_json::Value) -> Result<(), Error> {
    ensure!(v.is_object(), "Invalid JSON type for instruction");
    match v.as_object() {
        Some(o) => {
            match o.get("value") {
                Some(value) => {
                    if value.is_string() {
                        match value.as_str() {
                            Some(val_str) => {
                                let val_str_c = format!("{}", val_str);
                                match value::Value::from(val_str_c) {
                                    Ok(val) => {
                                        let mut vm = vm::BUND.lock().unwrap();
                                        vm.push(val);
                                        drop(vm);
                                    }
                                    Err(err) => {
                                        bail!("Invalid JSON for instruction: {}", err);
                                    }
                                }
                            }
                            None => {
                                bail!("Invalid JSON for instruction: Can not cast string");
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
