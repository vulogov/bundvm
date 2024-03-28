extern crate log;
use serde_json;
use crate::vm;
use rust_dynamic::value;
use easy_error::{bail, ensure, Error};

pub fn vm_json_push_lambda(v: serde_json::Value) -> Result<(), Error> {
    ensure!(v.is_object(), "Invalid JSON type for instruction");
    match v.as_object() {
        Some(_) => {
            let mut vm = vm::BUND.lock().unwrap();
            let res = vm.push(value::Value::lambda());
            drop(vm);
            return res;
        }
        None => {
            bail!("Invalid JSON for instruction: Can not cast object");
        }
    }
}
