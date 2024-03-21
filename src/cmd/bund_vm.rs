extern crate log;
use crate::cmd;
use crate::stdlib::{getfile, vm_execute};

pub fn run(_c: &cmd::Cli, vm: cmd::Vm)  {
    log::trace!("zbus_vm::run() reached");

    if vm.source.stdin {
        vm_execute::instructions(getfile::get_file_from_stdin());
    } else {
        match &vm.source.file {
            Some(script_name) => {
                match getfile::get_file_from_file(script_name.trim().to_string()) {
                    Some(script) => vm_execute::instructions(script),
                    None => log::error!("Instructions set is empty"),
                }
            }
            None => {
                match &vm.source.url {
                    Some(script_name) => {
                        match getfile::get_file_from_uri(script_name.trim().to_string()) {
                            Some(script) => vm_execute::instructions(script),
                            None => log::error!("Instructions set is empty"),
                        }
                    }
                    None => {
                        vm_execute::instructions(getfile::get_file_from_stdin());
                    }
                }
            }
        }
    }

}
