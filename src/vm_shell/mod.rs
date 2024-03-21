use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;

fn vm_shell_answer_function() -> i32 {
    42 as i32
}

pub fn init_vm_shell(e: &mut Engine) {
    log::debug!("Entering VM shell configuration");
    e.register_fn("answer", vm_shell_answer_function);
}
