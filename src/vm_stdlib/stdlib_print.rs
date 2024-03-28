extern crate log;
use easy_error::{bail, Error};
use crate::vm::BUNDCore;
use rust_dynamic::value::{Value};
use crate::vm::vm_applicatives::{BundApplicative, JUSTONE};

fn stdlib_print_fun(_ctx: &mut BUNDCore, _name: &str, value: Value) -> Result<Option<Value>, Error> {
    match value.cast_list() {
        Ok(l_value) => {
            for i in l_value {
                println!("{}", i);
            }
        }
        Err(err) => {
            bail!("{}", err);
        }
    }
    Ok(None)
}

fn stdlib_dump_fun(_ctx: &mut BUNDCore, _name: &str, value: Value) -> Result<Option<Value>, Error> {
    match value.cast_list() {
        Ok(l_value) => {
            for i in l_value {
                println!("{:?}", i);
            }
        }
        Err(err) => {
            bail!("{}", err);
        }
    }
    Ok(None)
}

pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library: print");
    ctx.register("print", BundApplicative::new("print", JUSTONE, stdlib_print_fun));
    ctx.register("dump", BundApplicative::new("dump", JUSTONE, stdlib_dump_fun));
}
