extern crate log;
use serde_json;
use crate::vm;
use crate::vm_json;
use easy_error::{bail, ensure, Error};

pub fn vm_json_execute(v: serde_json::Value) -> Result<(), Error> {
    ensure!(v.is_object(), "Invalid JSON type for instruction");
    match v.as_object() {
        Some(o) => {
            match o.get("value") {
                Some(value) => {
                    if value.is_string() {
                        match value.as_str() {
                            Some(val_str) => {
                                let mut vm = vm::BUND.lock().unwrap();
                                let is_app = vm.have_applicative();
                                drop(vm);
                                if is_app {
                                    return vm_json_call::vm_json_call(v);
                                } else {
                                    return vm_json_call_object::vm_json_push_call(v);
                                }
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
}
