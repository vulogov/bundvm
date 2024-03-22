extern crate log;
use crate::cmd;
use crate::vm_shell;
use crate::vm;
use crate::stdlib::{getfile, vm_execute};
use steel::steel_vm::engine::Engine;

pub fn run(_c: &cmd::Cli, vm: cmd::Vm)  {
    log::trace!("zbus_vm::run() reached");

    let mut bund = vm::BUND.lock().unwrap();
    bund.shell_if_error = vm.shell_if_error;
    log::debug!("If there is an error, VM will enter into interactive shell: {}", bund.shell_if_error);
    drop(bund);

    if vm.source.stdin {
        log::debug!("Ready to get VM instructions from stdin");
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
                        match &vm.source.eval {
                            Some(script) => vm_execute::instructions(script.to_string()),
                            None => {
                                if vm.inst_from_stdin {
                                    log::info!("Rolling down to reading VM instructions from stdin");
                                    cmd::bund_shell::vm_shell_cmd_execute(getfile::get_file_from_stdin());
                                } else {
                                    log::info!("Rolling down to reading VM instructions from stdin is disabled");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if vm.shell {
        log::debug!("Requesting of drop into an interactive shell");
        let mut e = Engine::new();
        vm_shell::init_vm_shell(&mut e);
        log::debug!("VM shell engine has been created");
        cmd::bund_shell::vm_interactive_shell_execute(&mut e, vm.nocolor);
    }
    if vm.vmstatus_at_exit {
        let mut bund = vm::BUND.lock().unwrap();
        bund.display_message("VM exiting".to_string(), "N/A".to_string());
        drop(bund);
    }
}
