extern crate log;
use crate::vm::BUNDCore;
use easy_error::{Error};
use rust_dynamic::value::{Value};
use rust_dynamic::types::{CALL, LAMBDA};

impl BUNDCore {
    pub fn push(&mut self, v: Value) -> Result<(), Error> {
        match v.dt {
            CALL => {
                log::debug!("CALL object detected, pushing to call stack");
                self.call_stack.push_back(v);
            }
            LAMBDA => {
                log::debug!("LAMBDA object detected, pushing to lambda stack");
                self.scaffold.push_back(v);
            }
            _ => {
                self.stack.push(v);
            }
        }
        Ok(())
    }
}
