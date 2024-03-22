use rust_twostack::ts::TS;
use lazy_static::lazy_static;
use std::sync::Mutex;
use zenoh::config::{Config};

pub mod vm_error;

#[derive(Clone)]
pub struct BUNDCore {
    pub version:        String,
    pub stack:          TS,
    // bus
    pub zc:             Config,
    // vm behavioral
    pub shell_if_error: bool,
}

impl BUNDCore {
    pub fn init() -> Self {
        Self {
            version:        env!("CARGO_PKG_VERSION").to_string(),
            stack:          TS::new(),
            zc:             Config::default(),
            shell_if_error: false,
        }
    }
}

lazy_static! {
    pub static ref BUND: Mutex<BUNDCore> = {
        let vm: Mutex<BUNDCore> = Mutex::new(BUNDCore::init());
        vm
    };
}
