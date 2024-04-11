extern crate log;
use easy_error::{bail, Error};
use crate::vm::BUNDCore;
use rust_dynamic::types;
use rust_dynamic::value::{Value};
use crate::vm::vm_applicatives::{BundApplicative, JUSTONE};

fn stdlib_execute_fun(ctx: &mut BUNDCore, _name: &str, value: Value) -> Result<Option<Value>, Error> {
    log::debug!("Executing element on the stack: {:?}", &value);
    match value.dt {
        types::PTR => {
            match value.cast_string() {
                Ok(ptr_name) => {
                    log::debug!("Calling ctx.call({})", &ptr_name);
                    match ctx.call(ptr_name) {
                        Ok(_) => {
                            return Ok(None);
                        }
                        Err(err) => {
                            bail!("{}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("{}", err);
                }
            }
        }
        _ => {
            match ctx.vm_route(value) {
                Ok(_) => {
                    return Ok(None);
                }
                Err(err) => {
                    bail!("{}", err);
                }
            }
        }
    }
}

pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library: execute");
    ctx.register("execute", BundApplicative::new("execute", JUSTONE, stdlib_execute_fun));
}
