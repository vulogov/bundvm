extern crate log;
use comfy_table::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use serde_json::json;
use crate::vm;
use crate::vm_json;
use crate::vm_shell;

pub fn vm_shell_push_lambda() -> Result<(), String> {
    log::debug!("Pushing lambda from shell");
    match vm_json::vm_json_lambda::vm_json_push_lambda(
        json!({
            "type":     "LAMBDA",
            "value":    null,
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
            return Err(format!("{}", err).into());
        }
    }
    Ok(())
}

pub fn vm_shell_display_lambdas() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(60);
    table.add_row(vec![
            Cell::new("List of registered lambdas").fg(Color::Blue),
    ]);
    let vm = vm::BUND.lock().unwrap();
    for l in vm.lambdas.keys() {
        table.add_row(vec![
                Cell::new(format!("{}", l)).fg(Color::White),
        ]);
    }
    drop(vm);
    println!("{}", table);
}

pub fn vm_shell_display_lambda(key: String) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(60);
    table.add_row(vec![
            Cell::new(format!("Content of lambda {}", &key)).fg(Color::Blue),
    ]);
    let vm = vm::BUND.lock().unwrap();
    if vm.lambdas.contains_key(&key) {
        match vm.lambdas.get(&key) {
            Some(v) => {
                match v.cast_lambda() {
                    Ok(l_value) => {
                        for i in l_value {
                            table.add_row(vec![
                                    Cell::new(format!("{:?}", i)).fg(Color::White),
                            ]);
                        }
                    }
                    Err(err) => {
                        log::error!("Casting over lamba content return error: {}", err);
                    }
                }
            }
            None => {
                log::error!("Lambda {} not found but it should be", &key);
            }
        }
    } else {
        log::error!("Lambda {} not found", &key);
        vm_shell_display_lambdas();
    }
    println!("{}", table);
}
