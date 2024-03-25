extern crate log;
use serde_json::json;
use crate::vm_json;
use crate::vm_shell;

pub fn vm_shell_call(s: String) -> Result<(), String> {
    log::debug!("Calling functor {} from shell", s);
    match vm_json::vm_json_call::vm_json_call(
        json!({
            "type":     "CALL",
            "value":    s,
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
