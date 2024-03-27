extern crate log;

pub mod banner;
pub mod getfile;
pub mod hostname;
pub mod vm_execute;
pub mod vm_execute_json;
pub mod vm_feed;

use crate::cmd::{Cli};


pub fn initlib(_c: &Cli) {
    log::debug!("Running STDLIB init");
}
