extern crate log;
use easy_error::{Error};
use crate::vm::BUNDCore;
use rust_dynamic::value::{Value};
use crate::vm::vm_applicatives::{BundApplicative, JUSTONE};

fn stdlib_terminate_lambda_fun(ctx: &mut BUNDCore, _name: &str, value: Value) -> Result<Option<Value>, Error> {
    log::debug!("Finishing lambda");
    match ctx.lambda_finish_scaffolding(value) {
        Ok(_) => {},
        Err(err) => return Err(err),
    }
    Ok(None)
}

pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library: lambda");
    ctx.register("endlambda", BundApplicative::new("endlambda", JUSTONE, stdlib_terminate_lambda_fun));
}
