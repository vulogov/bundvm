extern crate log;

pub mod banner;
pub mod getfile;
pub mod hostname;

use crate::cmd::{Cli};


pub fn initlib(_c: &Cli) {
    log::debug!("Running STDLIB init");
}
