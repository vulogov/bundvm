extern crate log;
use crate::cmd;
use crate::vm_shell;
use crate::stdlib::{getfile, banner};

use yansi::Paint;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Config, EditMode};
use steel::steel_vm::engine::Engine;



pub fn vm_shell_cmd_execute_in_engine(e: &mut Engine, cmd: String) {
    match e.run(cmd.as_str()) {
        Ok(res) => {
            log::trace!("VM shell returns: {:?}", res);
        }
        Err(err) => {
            log::trace!("VM shell returns error: {:?}", err);
        }
    }
}

pub fn vm_shell_cmd_execute(cmd: String) {
    log::debug!("Executing shell command: len={}", &cmd.len());
    let mut e = Engine::new();
    vm_shell::init_vm_shell(&mut e);
    log::debug!("VM shell engine has been created");
    vm_shell_cmd_execute_in_engine(&mut e, cmd);
}

pub fn vm_interactive_shell_execute(e: &mut Engine, nocolor: bool) {
    log::debug!("Entering interactive shell");
    println!("{}", banner::bund_banner());

    if ! nocolor {
        log::debug!("Enable colors in shell");
        Paint::enable_windows_ascii();
        Paint::enable();
    } else {
        log::debug!("Disable colors in shell");
        Paint::disable();
    }

    let rl_config = Config::builder()
        .history_ignore_space(true)
        .edit_mode(EditMode::Emacs)
        .build();

    let mut line = Editor::<()>::with_config(rl_config).unwrap();
    if line.load_history(".bundvm_history").is_err() {
        log::warn!("No previous history discovered");
    }

    loop {
        let prompt = format!("{}{}{}{}{} {} ", Paint::yellow("["), Paint::red("B"), Paint::blue("U").bold(), Paint::white("N"), Paint::cyan("D"), Paint::green(">").bold());
        let readline = line.readline(&prompt);
        match readline {
            Ok(l) => {
                vm_shell_cmd_execute_in_engine(e, l);
            },
            Err(ReadlineError::Interrupted) => {
                log::info!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                log::info!("CTRL-D");
                break
            },
            Err(err) => {
                log::error!("Error: {:?}", err);
                break
            }
        }
    }
    log::debug!("Now saving history");
    match line.save_history(".bundvm_history") {
        Ok(_) => {}
        Err(err) => log::error!("Error saving VM shell history file: {}", err),
    }
    println!("{}", banner::banner(&"Zay Gezunt".to_string()));
}

pub fn run(_c: &cmd::Cli, shell: cmd::Shell)  {
    log::trace!("zbus_shell::run() reached");
    let mut e = Engine::new();
    vm_shell::init_vm_shell(&mut e);
    log::debug!("VM shell engine has been created");

    if shell.source.stdin {
        vm_shell_cmd_execute_in_engine(&mut e, getfile::get_file_from_stdin())
    } else {
        match &shell.source.file {
            Some(script_name) => {
                match getfile::get_file_from_file(script_name.trim().to_string()) {
                    Some(script) => vm_shell_cmd_execute_in_engine(&mut e, script),
                    None => log::error!("Shell scipt is empty"),
                }
            }
            None => {
                match &shell.source.url {
                    Some(script_name) => {
                        match getfile::get_file_from_uri(script_name.trim().to_string()) {
                            Some(script) => vm_shell_cmd_execute_in_engine(&mut e, script),
                            None => log::error!("Shell scipt is empty"),
                        }
                    }
                    None => {
                        match &shell.source.eval {
                            Some(script) => vm_shell_cmd_execute_in_engine(&mut e, script.to_string()),
                            None => {
                                log::debug!("Shell commands pre-execution was not requested");
                            }
                        }
                    }
                }
            }
        }
    }
    vm_interactive_shell_execute(&mut e, shell.nocolor);
}
