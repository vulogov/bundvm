extern crate log;
use easy_error::{Error};
use crate::vm::BUNDCore;
use rust_dynamic::value::{Value};
use crate::vm::vm_applicatives::{BundApplicative, JUSTONE};

fn stdlib_print_fun(_ctx: &mut BUNDCore, _name: &str, _value: Value) -> Result<Option<Value>, Error> {
    Ok(None)
}

pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library: print");
    ctx.register("print", BundApplicative::new("print", JUSTONE, stdlib_print_fun));
}
