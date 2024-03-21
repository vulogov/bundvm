extern crate log;
use rust_dynamic::value::Value;

use crate::vm;

pub fn vm_shell_push_int(n: i64) {
    log::debug!("Pushing integer number {} into stack", n);
    let mut vm = vm::BUND.lock().unwrap();
    vm.stack.push(Value::from(n).expect("Value expected"));
    drop(vm);
}
