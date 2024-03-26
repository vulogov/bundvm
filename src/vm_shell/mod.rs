use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
use crate::vm;

pub mod vm_shell_status;
pub mod vm_shell_push;
pub mod vm_shell_pull;
pub mod vm_shell_core;
pub mod vm_shell_call;


fn vm_shell_answer_function() -> i32 {
    42 as i32
}

pub fn vm_shell_error(msg: String) {
    let mut bund = vm::BUND.lock().unwrap();
    bund.display_message(msg.to_string(), "N/A".to_string());
    drop(bund);
}

pub fn init_vm_shell(e: &mut Engine) {
    log::debug!("Entering VM shell configuration");
    e.register_fn("answer", vm_shell_answer_function);
    e.register_fn("quick-status", vm_shell_status::vm_shell_quick_status);
    e.register_fn("full-status", vm_shell_status::vm_shell_full_status);
    e.register_fn("push-integer", vm_shell_push::vm_shell_push_int);
    e.register_fn("push-float", vm_shell_push::vm_shell_push_float);
    e.register_fn("push-string", vm_shell_push::vm_shell_push_string);
    e.register_fn("push-true", vm_shell_push::vm_shell_push_bool_true);
    e.register_fn("push-false", vm_shell_push::vm_shell_push_bool_false);
    e.register_fn("push-list", vm_shell_push::vm_shell_push_empty_list);
    e.register_fn("pull", vm_shell_pull::vm_shell_pull);
    e.register_fn("pull-raw", vm_shell_pull::vm_shell_pull_raw);
    e.register_fn("vm-clear", vm_shell_core::vm_shell_clear);
    e.register_fn("vm-call", vm_shell_call::vm_shell_call);

}
