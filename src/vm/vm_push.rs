extern crate log;
use crate::vm::BUNDCore;
use easy_error::{Error};
use rust_dynamic::value::{Value};
use rust_dynamic::types::{CALL};

impl BUNDCore {
    pub fn push(&mut self, v: Value) -> Result<(), Error> {
        match v.dt {
            CALL => {
                log::debug!("CALL object detected, pushing to call stack");
                self.call_stack.push_back(v);
            }
            _ => {
                self.stack.push(v);
            }
        }
        Ok(())
    }
}
