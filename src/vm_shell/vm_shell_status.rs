extern crate log;
use comfy_table::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

use crate::vm;

pub fn vm_shell_quick_status() {
    log::debug!("Displaying quick status about VM");
    let mut vm = vm::BUND.lock().unwrap();
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(40);
    table.add_row(vec![
            "Version",
            &vm.version,
    ]);
    table.add_row(vec![
            "Number of stacks",
            format!("{}", vm.stack.len()).as_str(),
    ]);
    table.add_row(vec![
            "Number of elements in current stack",
            format!("{}", vm.stack.stack_len()).as_str(),
    ]);
    println!("{}", table);
    drop(vm);
}
