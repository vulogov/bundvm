extern crate log;
use crate::vm::BUNDCore;

pub mod stdlib_print;
pub mod stdlib_stack;
pub mod stdlib_lambda;
pub mod stdlib_terminate_and_execute;



pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library");
    stdlib_print::init_stdlib(ctx);
    stdlib_stack::init_stdlib(ctx);
    stdlib_terminate_and_execute::init_stdlib(ctx);
    stdlib_lambda::init_stdlib(ctx);
}
