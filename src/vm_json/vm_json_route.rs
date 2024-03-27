extern crate log;
use serde_json;
use crate::vm_json::*;
use easy_error::{bail, ensure, Error};

pub fn vm_json_route(obj: serde_json::Value) -> Result<(), Error> {
    ensure!(obj.is_object(), "Invalid JSON type for instruction");
    match &obj.as_object() {
        Some(m_obj) => {
            match m_obj.get("type") {
                Some(i_type) => {
                    match i_type.as_str() {
                        Some(type_str) => {
                            match type_str {
                                "INTEGER" => {
                                    return vm_json_numbers::vm_json_numbers_push_integer(obj);
                                }
                                "FLOAT" => {
                                    return vm_json_numbers::vm_json_numbers_push_float(obj);
                                }
                                "STRING" => {
                                    return vm_json_string::vm_json_numbers_push_string(obj);
                                }
                                "BOOLEAN" => {
                                    return vm_json_bool::vm_json_numbers_push_bool(obj);
                                }
                                "SEPARATOR" => {
                                    return vm_json_separator::vm_json_push_separator(obj);
                                }
                                "LIST" => {
                                    return vm_json_list::vm_json_push_list(obj);
                                }
                                _ => {
                                    bail!("JSON: Unknown instruction type: {}", type_str)
                                }
                            }
                        }
                        None => {
                            bail!("JSON: unknown type format: {}", i_type);
                        }
                    }
                }
                None => {
                    bail!("JSON: Instruction do not have a type. Not possible!");
                }
            }
        }
        None => {
            bail!("JSON: is this an object or it is not?");
        }
    }
}
