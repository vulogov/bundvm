extern crate log;
use rust_dynamic::value::{Value};
use rust_dynamic::ctx::{Context, CtxApplicative, JUSTONE};

fn stdlib_print_fun(_ctx: &dyn Context, _name: &str, _value: Value) -> Option<Value> {
    None
}

pub fn init_stdlib(ctx: &mut dyn Context) {
    log::debug!("Init VM standard library: print");
    ctx.register("print", CtxApplicative::new("print", JUSTONE, stdlib_print_fun));
}
