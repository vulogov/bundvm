extern crate log;
use crate::cmd;
use crate::stdlib::{getfile, vm_execute};
use steel::steel_vm::engine::Engine;

pub fn run(_c: &cmd::Cli, vm: cmd::Vm)  {
    log::trace!("zbus_vm::run() reached");
    let mut e = Engine::new();
    log::debug!("VM shell engine has been created");

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
                        cmd::bund_shell::vm_shell_cmd_execute(getfile::get_file_from_stdin());
                    }
                }
            }
        }
    }
    if vm.shell {
        log::debug!("Requesting of drop into an interactive shell");
        cmd::bund_shell::vm_interactive_shell_execute(&mut e, vm.nocolor);
    }
}
