extern crate log;
use crate::cmd;
use crate::stdlib::vm_execute_json::{run_json_instructions};
use easy_error::{ensure, Error};

pub fn instructions(inst: String, vm_args: cmd::Vm) -> Result<(), Error> {
    ensure!(inst.len() == 0, "VM: No instructions has been passed");

    match vm_args.instruction_type {
        cmd::InstructionType::Json => {
            return run_json_instructions(inst);
        }
    }
}
