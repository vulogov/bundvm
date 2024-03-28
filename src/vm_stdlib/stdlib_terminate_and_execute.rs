extern crate log;
use std::collections::VecDeque;
use easy_error::{bail, ensure, Error};
use crate::vm::BUNDCore;
use rust_dynamic::value::{Value};
use rust_dynamic::types::{LIST, FIFO, QUEUE, NONE, CALL};
use crate::vm::vm_applicatives::{BundApplicative, NOEXTRA};

fn stdlib_terminate_and_execute_fun(ctx: &mut BUNDCore, _name: &str, _value: Value) -> Result<Option<Value>, Error> {
    if ctx.is_callstack_nonempty() {
        log::debug!("Elements has been detrected in callstack, execute them first");
        match ctx.call_stack.pop_back() {
            Some(value) => {
                ensure!(value.dt == CALL, "Invalid JSON type for instruction");
                match value.cast_string() {
                    Ok(f_call) => {
                        match ctx.call(f_call) {
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
            None => {
                bail!("Something fishy, call stack is empty or not!");
            }
        }
    }
    ensure!(ctx.stack.stack_len() > 0, "VM: stack depth is not deep enough for that operation");
    let mut scaffold: VecDeque<Value> = VecDeque::new();
    loop {
        match ctx.stack.pull() {
            Some(mut value) => {
                match value.dt {
                    LIST | FIFO | QUEUE => {
                        loop {
                            match scaffold.pop_front() {
                                Some(pvalue) => {
                                    value = value.push(pvalue);
                                }
                                None => {
                                    return Ok(Some(value));
                                }
                            }
                        }
                    }
                    NONE => {
                        log::debug!("Separator detected. Creating new stack and push scaffold there");
                        ctx.stack.add_stack();
                        loop {
                            match scaffold.pop_front() {
                                Some(pvalue) => {
                                    let _ = ctx.push(pvalue);
                                }
                                None => {
                                    return Ok(None);
                                }
                            }
                        }
                    }
                    _ => {
                        scaffold.push_back(value);
                    }
                }
            }
            None => {
                bail!("VM: during execution stack exhausted before target was acquired");
            }
        }
    }
}

pub fn init_stdlib(ctx: &mut BUNDCore) {
    log::debug!("Init VM standard library: terminate_and_execute");
    ctx.register("done", BundApplicative::new("done", NOEXTRA, stdlib_terminate_and_execute_fun));
}
