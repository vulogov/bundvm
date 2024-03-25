extern crate log;
use rust_dynamic::value::{Value};
use rust_dynamic::ctx::{Context, CtxApplicative, NOEXTRA};

fn stdlib_stack_drop_fun(_ctx: &dyn Context, _name: &str, _value: Value) -> Option<Value> {
    log::debug!("Dropping top value from stack");
    None
}

pub fn init_stdlib(ctx: &mut dyn Context) {
    log::debug!("Init VM standard library: stack");
    ctx.register("drop", CtxApplicative::new("drop", NOEXTRA, stdlib_stack_drop_fun));
    ctx.register("^", CtxApplicative::new("^", NOEXTRA, stdlib_stack_drop_fun));
}
