extern crate log;
use comfy_table::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

use crate::vm;

pub fn vm_shell_pull() {
    log::debug!("Pulling value from stack");
    let mut vm = vm::BUND.lock().unwrap();
    match vm.stack.pull() {
        Some(value) => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(40);
            table.add_row(vec![
                    "Returned value",
                    format!("{}", value).as_str(),
            ]);
            println!("{}", table);
        }
        None => {
            log::info!("Stack is empty");
        }
    }
    drop(vm);
}

pub fn vm_shell_pull_raw() {
    log::debug!("Pulling value from stack");
    let mut vm = vm::BUND.lock().unwrap();
    match vm.stack.pull() {
        Some(value) => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(60);
            table.add_row(vec![
                    "Returned value",
                    format!("{:?}", value).as_str(),
            ]);
            println!("{}", table);
        }
        None => {
            log::info!("Stack is empty");
        }
    }
    drop(vm);
}
