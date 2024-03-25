extern crate log;
use crate::vm::BUNDCore;

pub mod stdlib_print;
pub mod stdlib_stack;


pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library");
    stdlib_print::init_stdlib(ctx);
    stdlib_stack::init_stdlib(ctx);
}
