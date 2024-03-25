extern crate log;
use rust_dynamic::ctx::{Context};

pub mod stdlib_print;
pub mod stdlib_stack;


pub fn init_stdlib(ctx: &mut dyn Context) {
    log::debug!("Init VM standard library");
    stdlib_print::init_stdlib(ctx);
}
