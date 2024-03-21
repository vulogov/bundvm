extern crate log;
use env_logger::{Env};
use crate::cmd;


pub fn setloglevel(c: &cmd::Cli) {
    let env = Env::default().filter_or("BUND_LOG_LEVEL", "error");
    match c.debug {
        0 => {
            env_logger::init_from_env(env);
            log::debug!("Set loglevel from environment");
        }
        1 => {
            let env = Env::default()
                .filter_or("BUND_LOG_LEVEL", "bundvm=info");
            env_logger::init_from_env(env);
            log::debug!("Set loglevel=info");
        }
        2 => {
            let env = Env::default()
                .filter_or("BUND_LOG_LEVEL", "bundvm=debug");
            env_logger::init_from_env(env);
            log::debug!("Set loglevel=debug");
        }
        _ => {
            let env = Env::default()
                .filter_or("BUND_LOG_LEVEL", "bundvm=trace");
            env_logger::init_from_env(env);
            log::debug!("Set loglevel=trace");
        }
    }
    log::debug!("setloglevel::setloglevel() reached")
}
