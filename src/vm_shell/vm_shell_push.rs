extern crate log;
use serde_json::json;
use crate::vm_json;
use crate::vm_shell;

pub fn vm_shell_push_int(n: i64) {
    log::debug!("Pushing integer number {} into stack", n);
    match vm_json::vm_json_numbers::vm_json_numbers_push_integer(
        json!({
            "type":     "INTEGER",
            "value":    n,
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}

pub fn vm_shell_push_float(n: f64) {
    log::debug!("Pushing float number {} into stack", n);
    match vm_json::vm_json_numbers::vm_json_numbers_push_float(
        json!({
            "type":     "FLOAT",
            "value":    n,
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}

pub fn vm_shell_push_string(s: String) {
    log::debug!("Pushing string {} into stack", s);
    match vm_json::vm_json_string::vm_json_numbers_push_string(
        json!({
            "type":     "STRING",
            "value":    s.clone(),
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}

pub fn vm_shell_push_bool(b: bool) {
    log::debug!("Pushing bool {} into stack", b);
    match vm_json::vm_json_bool::vm_json_numbers_push_bool(
        json!({
            "type":     "BOOLEAN",
            "value":    b,
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}
