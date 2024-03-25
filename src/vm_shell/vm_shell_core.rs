extern crate log;

use crate::vm;

pub fn vm_shell_clear() {
    log::debug!("Clearing the stack");
    let mut vm = vm::BUND.lock().unwrap();
    vm.stack.clear();
    drop(vm);
}
