extern crate log;
use easy_error::{ensure, bail, Error};
use crate::vm::BUNDCore;
use crate::vm::vm_applicatives::{BundApplicative, NOEXTRA};
use rust_dynamic::value::{Value};

fn stdlib_stack_drop_fun(ctx: &mut BUNDCore, _name: &str, _value: Value) -> Result<Option<Value>, Error> {
    log::debug!("Dropping top value from stack");
    ensure!(ctx.stack.stack_len() > 0, "VM: stack is not deep enough for that operation");
    if ctx.stack.stack_len() > 0 {
        let _ = ctx.stack.pull();
    }
    Ok(None)
}

fn stdlib_stack_dup_fun(ctx: &mut BUNDCore, _name: &str, _value: Value) -> Result<Option<Value>, Error> {
    log::debug!("Duplicate top value on stack");
    ensure!(ctx.stack.stack_len() > 0, "VM: stack is not deep enough for that operation");
    if ctx.stack.stack_len() > 0 {
        let _ = ctx.stack.dup();
    }
    Ok(None)
}

fn stdlib_stack_swap_fun(ctx: &mut BUNDCore, _name: &str, _value: Value) -> Result<Option<Value>, Error> {
    log::debug!("Swapping top values on stack");
    ensure!(ctx.stack.stack_len() >= 2, "VM: stack is not deep enough for that operation");
    if ctx.stack.stack_len() > 0 {
        let _ = ctx.stack.swap();
    }
    Ok(None)
}

fn stdlib_stack_rotate_left_fun(ctx: &mut BUNDCore, _name: &str, _value: Value) -> Result<Option<Value>, Error> {
    log::debug!("Rotating stack to the left");
    ensure!(ctx.stack.stack_len() > 0, "VM: stack is not deep enough for that operation");
    if ctx.stack.stack_len() > 0 {
        match ctx.stack.current() {
            Some(stack) => {
                stack.left();
            }
            None => {
                bail!("VM: unable to identify current stack");
            }
        }
    }
    Ok(None)
}

fn stdlib_stack_rotate_right_fun(ctx: &mut BUNDCore, _name: &str, _value: Value) -> Result<Option<Value>, Error> {
    log::debug!("Rotating stack to the right");
    ensure!(ctx.stack.stack_len() > 0, "VM: stack is not deep enough for that operation");
    if ctx.stack.stack_len() > 0 {
        match ctx.stack.current() {
            Some(stack) => {
                stack.right();
            }
            None => {
                bail!("VM: unable to identify current stack");
            }
        }
    }
    Ok(None)
}

pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library: stack");
    ctx.register("drop", BundApplicative::new("drop", NOEXTRA, stdlib_stack_drop_fun));
    ctx.register("^", BundApplicative::new("^", NOEXTRA, stdlib_stack_drop_fun));
    ctx.register("dup", BundApplicative::new("dup", NOEXTRA, stdlib_stack_dup_fun));
    ctx.register("swap", BundApplicative::new("swap", NOEXTRA, stdlib_stack_swap_fun));
    ctx.register("rotate-left", BundApplicative::new("rotate-left", NOEXTRA, stdlib_stack_rotate_left_fun));
    ctx.register("rotate-right", BundApplicative::new("rotate-left", NOEXTRA, stdlib_stack_rotate_right_fun));

}
