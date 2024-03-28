extern crate log;
use crate::vm;
use crate::stdlib;
use crate::vm_json::vm_json_route::{vm_json_route};
use serde_json;
use easy_error::{bail, Error};

pub fn run_json_instructions(inst: String) -> Result<(), Error> {
    for instruction in inst.lines() {
        let (first_chair, _) =  stdlib::string::car_cdr(&instruction);
        if first_chair == "#" {
            continue;
        }
        match serde_json::from_str::<serde_json::Value>(instruction) {
            Ok(json_inst) => {
                log::trace!("Instruction: {:?}", &json_inst);
                match vm_json_route(json_inst.clone()) {
                    Ok(_) => {},
                    Err(err) => {
                        let mut vm = vm::BUND.lock().unwrap();
                        vm.display_message(format!("{}", err), instruction.to_string());
                        drop(vm);
                        bail!("{}", err);
                    }
                }
            }
            Err(err) => {
                let mut vm = vm::BUND.lock().unwrap();
                vm.display_message(format!("{}", err), instruction.to_string());
                drop(vm);
                bail!("VM: error parsing JSON instruction: {}", err);
            }
        }
    }
    Ok(())
}
