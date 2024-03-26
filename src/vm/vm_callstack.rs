extern crate log;
use crate::vm::BUNDCore;
use easy_error::{Error};

impl BUNDCore {
    pub fn is_callstack_nonempty(&mut self) -> Result<bool, Error> {
        Ok(self.call_stack.len() > 0)
    }

}
