extern crate log;
use crate::vm::BUNDCore;
use easy_error::{bail, Error};
use rust_dynamic::value::{Value};
use rust_dynamic::types;

impl BUNDCore {
    pub fn vm_route(&mut self, obj: Value) -> Result<(), Error> {
        match obj.dt {
            types::INTEGER | types::FLOAT | types::STRING | types::BOOL | types::LIST | types::PTR => {
                return self.push(obj);
            }
            types::LAMBDA => {
                return self.push(obj);
            }
            types::CALL => {
                return self.push(obj);
            }
            _ => {
                bail!("Object({}) that has been passed to route have unsupported type: {}", obj.id, obj.dt);
            }
        }
    }
}
