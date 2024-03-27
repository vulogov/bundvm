extern crate log;
use crate::vm::BUNDCore;

impl BUNDCore {
    pub fn is_callstack_nonempty(&mut self) -> bool {
        self.call_stack.len() > 0
    }

}
