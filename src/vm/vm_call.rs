extern crate log;
use crate::vm::BUNDCore;

impl BUNDCore {
    pub fn call(&mut self, fun: String) {
        log::debug!("BUND VM: calling functor: {}", &fun);
    }
}
