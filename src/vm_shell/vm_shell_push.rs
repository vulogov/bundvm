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

pub fn vm_shell_push_bool_true() {
    log::debug!("Pushing bool true into stack");
    match vm_json::vm_json_bool::vm_json_numbers_push_bool(
        json!({
            "type":     "BOOLEAN",
            "value":    true,
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}

pub fn vm_shell_push_bool_false() {
    log::debug!("Pushing bool false into stack");
    match vm_json::vm_json_bool::vm_json_numbers_push_bool(
        json!({
            "type":     "BOOLEAN",
            "value":    false,
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}

pub fn vm_shell_push_empty_list() {
    log::debug!("Pushing empty list into stack");
    match vm_json::vm_json_list::vm_json_push_list(
        json!({
            "type":     "LIST",
            "value":    null,
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}

pub fn vm_shell_push_call(s: String) {
    log::debug!("Pushing call {} into VM", s);
    match vm_json::vm_json_call_object::vm_json_numbers_push_call(
        json!({
            "type":     "CALL",
            "value":    s.clone(),
        })
    ) {
        Ok(_) => {}
        Err(err) => {
            vm_shell::vm_shell_error(format!("{}", err).to_string());
        }
    }
}
