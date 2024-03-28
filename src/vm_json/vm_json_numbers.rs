extern crate log;
use serde_json;
use crate::vm;
use rust_dynamic::value;
use easy_error::{bail, ensure, Error};

pub fn vm_json_numbers_push_integer(v: serde_json::Value) -> Result<(), Error> {
    ensure!(v.is_object(), "Invalid JSON type for instruction");
    match v.as_object() {
        Some(o) => {
            match o.get("value") {
                Some(value) => {
                    if value.is_i64() {
                        match value.as_i64() {
                            Some(val_64) => {
                                match value::Value::from(val_64) {
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
                                bail!("Invalid JSON for instruction: Can not cast i64 number");
                            }
                        }
                    } else {
                        bail!("Invalid JSON for instruction: value not an integer");
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

pub fn vm_json_numbers_push_float(v: serde_json::Value) -> Result<(), Error> {
    ensure!(v.is_object(), "Invalid JSON type for instruction");
    match v.as_object() {
        Some(o) => {
            match o.get("value") {
                Some(value) => {
                    if value.is_f64() {
                        match value.as_f64() {
                            Some(val_64) => {
                                match value::Value::from(val_64) {
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
                                bail!("Invalid JSON for instruction: Can not cast f64 number");
                            }
                        }
                    } else {
                        bail!("Invalid JSON for instruction: value not an float");
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
